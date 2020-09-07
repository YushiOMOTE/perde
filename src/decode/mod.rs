use crate::{schema::*, util::*};
use pyo3::prelude::*;
use serde::de::{self, DeserializeSeed, Deserializer, SeqAccess, Visitor};
use std::fmt;

pub mod class;
pub mod dict;
pub mod enums;
pub mod list;
pub mod object;
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
        match self {
            Schema::Primitive(p) => p.deserialize(deserializer),
            Schema::Dict(d) => d.deserialize(deserializer),
            Schema::List(l) => l.deserialize(deserializer),
            Schema::Set(s) => s.deserialize(deserializer),
            Schema::Tuple(t) => t.deserialize(deserializer),
            Schema::Class(c) => c.deserialize(deserializer),
            Schema::Enum(e) => e.deserialize(deserializer),
            Schema::Optional(o) => o.deserialize(deserializer),
            Schema::Union(u) => u.deserialize(deserializer),
        }
    }
}
