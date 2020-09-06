use crate::{schema::*, util::*};
use pyo3::{prelude::*, types::PyDict};
use serde::de::{self, DeserializeSeed, Deserializer, SeqAccess, Visitor};
use std::{collections::HashMap, fmt};

pub struct ListVisitor<'a>(pub &'a List);

impl<'a, 'de> Visitor<'de> for ListVisitor<'a> {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a list")
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut items = vec![];

        while let Some(value) = seq.next_element_seed(&*self.0.value)? {
            items.push(value);
        }

        Ok(items.to_object(py()))
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a List {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(ListVisitor(self))
    }
}
