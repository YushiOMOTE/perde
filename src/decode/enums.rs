use crate::{
    schema::*,
    types::{self, Object},
    util::*,
};
use pyo3::prelude::*;
use serde::de::{self, DeserializeSeed, Deserializer, Visitor};
use std::fmt;

struct EnumVisitor<'a>(&'a Enum);

impl<'a> EnumVisitor<'a> {
    #[cfg_attr(feature = "perf", flame)]
    fn vars(&self) -> Vec<&str> {
        self.0.variants.iter().map(|(v, _)| v.as_ref()).collect()
    }

    #[cfg_attr(feature = "perf", flame)]
    fn get<E>(&self, s: &str) -> Result<Object, E>
    where
        E: de::Error,
    {
        self.0
            .variants
            .get(s)
            .ok_or_else(|| {
                de::Error::custom(format!(
                    "the enum value must be any of {:?}: got `{}`",
                    self.vars(),
                    s
                ))
            })
            .and_then(|_| self.0.ty.value(s).map_err(de).map(|v| v.into()))
    }
}

impl<'a, 'de> Visitor<'de> for EnumVisitor<'a> {
    type Value = Object;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an enum value: {:?}", self.vars())
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_char<E>(self, value: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.get(&value.to_string())
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.get(value)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.get(value)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.get(&value)
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a Enum {
    type Value = Object;

    #[cfg_attr(feature = "perf", flame)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(EnumVisitor(self))
    }
}
