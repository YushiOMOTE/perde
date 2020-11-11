use crate::{
    error::{Convert, Result},
    schema::*,
    types::{self, Object},
};
use serde::de::{self, DeserializeSeed, Deserializer, IgnoredAny, MapAccess, Visitor};
use std::{borrow::Cow, collections::HashMap, fmt};

pub struct ClassVisitor<'a>(pub &'a Class);

impl<'a, 'de> Visitor<'de> for ClassVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a class")
    }

    fn visit_map<M>(self, mut access: M) -> std::result::Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        if self.0.flatten_fields.is_empty() {
            let mut tuple = types::Tuple::new(self.0.fields.len()).de()?;
            let mut setcount = 0;

            while let Some(key) = access.next_key()? {
                let key: Cow<str> = key;

                if let Some(s) = self.0.field(&key).de()? {
                    let value: Object = access.next_value_seed(&s.schema)?;

                    tuple.set(s.pos, value);
                    setcount += 1;
                } else {
                    let _: IgnoredAny = access.next_value()?;
                }
            }

            if setcount != self.0.num_fields() {
                return Err(de::Error::custom("missing field"));
            }

            self.0.ty.construct(tuple).de()
        } else {
            let mut map = HashMap::new();

            while let Some(key) = access.next_key()? {
                let key: Cow<str> = key;

                if let Some(s) = self.0.field(&key).de()? {
                    let value: Object = access.next_value_seed(&s.schema)?;

                    map.insert(key, value);
                } else {
                    let _: IgnoredAny = access.next_value()?;
                }
            }

            let cls = self.0.call(&mut map).de()?;

            if !map.is_empty() && self.0.attr.deny_unknown_fields {
                // TODO: Error
            }

            Ok(cls)
        }
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a Class {
    type Value = Object;

    fn deserialize<D>(self, deserializer: D) -> std::result::Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ClassVisitor(self))
    }
}

impl Class {
    pub fn num_fields(&self) -> usize {
        if self.flatten_fields.is_empty() {
            self.fields.len()
        } else {
            self.flatten_fields.len()
        }
    }

    pub fn field(&self, name: &str) -> Result<Option<&FieldSchema>> {
        let map = if self.flatten_fields.is_empty() {
            &self.fields
        } else {
            &self.flatten_fields
        };

        map.get(name)
            .map(|v| {
                if v.attr.skip || v.attr.skip_deserializing {
                    Ok(None)
                } else {
                    Ok(Some(v))
                }
            })
            .unwrap_or_else(|| {
                if self.attr.deny_unknown_fields {
                    Err(err!("the field `{}` is missing", name,))
                } else {
                    Ok(None)
                }
            })
    }

    pub fn call(&self, map: &mut HashMap<Cow<str>, Object>) -> Result<Object> {
        let args: Result<Vec<_>> = self
            .fields
            .iter()
            .map(|(k, s)| {
                if s.attr.flatten {
                    match &s.schema {
                        Schema::Class(cls) => return cls.call(map),
                        Schema::Dict(_) => {
                            let map = std::mem::replace(map, HashMap::new());
                            let mut dict = types::Dict::new()?;
                            for (k, v) in map {
                                dict.set(Object::new_str(&k)?, v)?;
                            }
                            return Ok(dict.into_inner());
                        }
                        _ => return Err(err!("found `flatten` attribute an non-map type",)),
                    }
                }

                let k: &str = k.as_ref();
                match map.remove(k) {
                    Some(v) => Ok(v),
                    None => {
                        if self.attr.default
                            || s.attr.default.is_some()
                            || s.attr.skip
                            || s.attr.skip_deserializing
                        {
                            if let Some(d) = s.attr.default.as_ref() {
                                return Ok(d.clone());
                            } else if let Some(d) = s.attr.default_factory.as_ref() {
                                return d.call_noarg();
                            }
                        }
                        Err(err!("missing field \"{}\"", k))
                    }
                }
            })
            .collect();

        self.construct(args?)
    }

    pub fn construct(&self, args: Vec<Object>) -> Result<Object> {
        let mut tuple = types::Tuple::new(args.len())?;
        for (i, arg) in args.into_iter().enumerate() {
            tuple.set(i, arg);
        }
        self.ty.construct(tuple)
    }
}
