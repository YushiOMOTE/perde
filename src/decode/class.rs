use crate::{schema::*, util::*};
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple, PyType},
};
use serde::de::{self, DeserializeSeed, Deserializer, IgnoredAny, MapAccess, Visitor};
use std::{collections::HashMap, fmt};

pub struct ClassVisitor<'a>(pub &'a Class);

impl<'a, 'de> Visitor<'de> for ClassVisitor<'a> {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a class")
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut args = HashMap::new();

        while let Some(key) = access.next_key()? {
            let key: &str = key;

            if let Some(s) = self.0.field(key)? {
                let value: PyObject = access.next_value_seed(&s.schema)?;

                args.insert(key, value);
            } else {
                let _: IgnoredAny = access.next_value()?;
            }
        }

        self.0.call(&mut args)
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a Class {
    type Value = PyObject;

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
                    Err(pyerr(format!(
                        "the field `{}` in `{}` type is missing",
                        name,
                        self.ty.as_ref(py()).name()
                    )))
                } else {
                    Ok(None)
                }
            })
            .map_err(de)
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn call<'a, E>(&self, map: &mut HashMap<&'a str, PyObject>) -> Result<PyObject, E>
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
                        Schema::Dict(dict) => {
                            let map = std::mem::replace(map, HashMap::new());
                            return Ok(map.into_py(py()));
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
                                return Ok(d.as_ref(py()).into());
                            } else if let Some(d) = s.attr.default_factory.as_ref() {
                                return d.as_ref(py()).call0().map(|v| v.into()).map_err(de);
                            }
                        }
                        Err(de::Error::custom(format!("missing field \"{}\"", k)))
                    }
                }
            })
            .collect();

        self.fastcall(args?)
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn fastcall<E>(&self, args: Vec<PyObject>) -> Result<PyObject, E>
    where
        E: de::Error,
    {
        macro_rules! to_tuple {
            ($args:expr, $($t:tt),*) => {{
                let mut args = $args.into_iter();
                $(let $t = args.next().unwrap();)*
                self.ty
                    .as_ref(py())
                    .call1(($($t,)*))
                    .map(|v| v.into())
            }}
        }

        match args.len() {
            0 => self.ty.as_ref(py()).call0().map(|v| v.into()),
            1 => to_tuple!(args, a),
            2 => to_tuple!(args, a, b),
            3 => to_tuple!(args, a, b, c),
            4 => to_tuple!(args, a, b, c, d),
            5 => to_tuple!(args, a, b, c, d, e),
            6 => to_tuple!(args, a, b, c, d, e, f),
            7 => to_tuple!(args, a, b, c, d, e, f, g),
            8 => to_tuple!(args, a, b, c, d, e, f, g, h),
            9 => to_tuple!(args, a, b, c, d, e, f, g, h, i),
            _ => {
                let args = PyTuple::new(py(), args);
                self.ty.as_ref(py()).call1(args).map(|v| v.into())
            }
        }
        .map_err(de)
    }
}
