use crate::{schema::*, util::*};
use pyo3::prelude::*;
use serde::de::{self, DeserializeSeed, Deserializer, SeqAccess, Visitor};
use std::fmt;

pub mod class;
pub mod dict;
pub mod enums;
pub mod list;
pub mod option;
pub mod primitive;
pub mod set;
pub mod tuple;
pub mod union;

impl<'a, 'de> DeserializeSeed<'de> for &'a Schema {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        unimplemented!()
    }
}
