use crate::{
    error::{Convert, Result},
    schema::*,
    types::{self, Object},
};
use serde::de::{DeserializeSeed, Deserializer, IgnoredAny, MapAccess, Visitor};
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

        Ok(cls)
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
                    Err(err!("unknown field `{}`", name,))
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
                        _ => return Err(err!("cannot use `flatten` attribute with an non-map type")),
                    }
                }

                if s.attr.skip || s.attr.skip_deserializing {
                    // Here we don't check if the field exists.
                    // Just use default, default_factory or default constructor.

                    if let Some(obj) = s.attr.default.as_ref() {
                        return Ok(obj.owned());
                    } else if let Some(obj) = s.attr.default_factory.as_ref() {
                        return obj.call_noarg();
                    } else if self.attr.default || s.attr.default_construct {
                        return Object::new_default(&s.schema);
                    } else {
                        bail!("`default` must be set together with `skip`/`skip_deserializing` attribute")
                    }
                }

                let k: &str = k.as_ref();
                match map.remove(k) {
                    Some(v) => Ok(v),
                    None => {
                        // Here field is missing.
                        // If default is defined, use it.
                        // If this is optional, return `None`.
                        if let Some(d) = s.attr.default.as_ref() {
                            return Ok(d.clone());
                        } else if let Some(d) = s.attr.default_factory.as_ref() {
                            return d.call_noarg();
                        } else if s.schema.is_optional() {
                            return Ok(Object::new_none());
                        } else if self.attr.default || s.attr.default_construct {
                            return Object::new_default(&s.schema);
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
