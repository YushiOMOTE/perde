use crate::{schema::*, util::*};
use pyo3::{
    prelude::*,
    types::{PyTuple, PyType},
};
use serde::de::{self, DeserializeSeed, Deserializer, IgnoredAny, MapAccess, Visitor};
use std::{collections::HashMap, fmt};

pub struct OptionVisitor<'a>(pub &'a Optional);

impl<'a, 'de> Visitor<'de> for OptionVisitor<'a> {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an option")
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(().to_object(py()))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        (&*self.0.value).deserialize(deserializer)
    }
}
