use crate::{
    schema::*,
    types::{self, Object},
    util::*,
};
use pyo3::prelude::*;
use serde::de::{self, DeserializeSeed, Deserializer, Visitor};
use std::fmt;

pub struct OptionVisitor<'a>(pub &'a Optional);

impl<'a, 'de> Visitor<'de> for OptionVisitor<'a> {
    type Value = Object;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an option")
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Object::new_none())
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        (&*self.0.value).deserialize(deserializer)
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a Optional {
    type Value = Object;

    #[cfg_attr(feature = "perf", flame)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(OptionVisitor(self))
    }
}
