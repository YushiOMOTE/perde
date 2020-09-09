use crate::{
    schema::*,
    types::{self, Object},
    util::*,
};
use pyo3::conversion::AsPyPointer;
use pyo3::{prelude::*, types::PyTuple};
use serde::de::{self, DeserializeSeed, Deserializer, IgnoredAny, MapAccess, Visitor};
use std::{collections::HashMap, fmt};

pub struct ClassVisitor<'a>(pub &'a Class);

impl<'a, 'de> Visitor<'de> for ClassVisitor<'a> {
    type Value = Object;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a class")
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut tuple = types::Tuple::new(self.0.fields.len()).map_err(de)?;

        while let Some(key) = access.next_key()? {
            let key: &str = key;

            if let Some(s) = self.0.field(key)? {
                let value: Object = access.next_value_seed(&s.schema)?;

                tuple.set(s.pos, value);
            } else {
                let _: IgnoredAny = access.next_value()?;
            }
        }

        self.0.ty.construct(tuple).map_err(de)
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a Class {
    type Value = Object;

    #[cfg_attr(feature = "perf", flame)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(ClassVisitor(self))
    }
}

impl Class {
    #[cfg_attr(feature = "perf", flame)]
    pub fn field<E>(&self, name: &str) -> Result<Option<&FieldSchema>, E>
    where
        E: de::Error,
    {
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
                    Err(pyerr(format!("the field `{}` is missing", name,)))
                } else {
                    Ok(None)
                }
            })
            .map_err(de)
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn call<'a, E>(&self, map: &mut HashMap<&'a str, Object>) -> Result<Object, E>
    where
        E: de::Error,
    {
        let args: Result<Vec<_>, _> = self
            .fields
            .iter()
            .map(|(k, s)| {
                if s.attr.flatten {
                    match &s.schema {
                        Schema::Class(cls) => return cls.call(map),
                        Schema::Dict(_) => {
                            let map = std::mem::replace(map, HashMap::new());

                            let mut dict = types::Dict::new().map_err(de)?;
                            for (k, v) in map {
                                dict.set(types::py_str(&k).map_err(de)?, v).map_err(de)?;
                            }
                            return Ok(dict.into_inner());
                        }
                        _ => {
                            return Err(de::Error::custom(
                                "found `flatten` attribute an non-map type",
                            ))
                        }
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
                                unimplemented!()
                            // return Ok(d.as_ref(py()).as_ptr());
                            } else if let Some(d) = s.attr.default_factory.as_ref() {
                                // return d.as_ref(py()).call0().map(|v| v.into()).map_err(de);
                                unimplemented!()
                            }
                        }
                        Err(de::Error::custom(format!("missing field \"{}\"", k)))
                    }
                }
            })
            .collect();

        self.construct(args?)
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn construct<E>(&self, args: Vec<Object>) -> Result<Object, E>
    where
        E: de::Error,
    {
        let mut tuple = types::Tuple::new(args.len()).map_err(de)?;
        for (i, arg) in args.into_iter().enumerate() {
            tuple.set(i, arg);
        }
        self.ty.construct(tuple).map_err(de)
    }
}
