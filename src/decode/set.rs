use crate::{schema::*, util::*};
use pyo3::prelude::*;
use serde::de::{DeserializeSeed, Deserializer, SeqAccess, Visitor};
use std::fmt;

pub struct SetVisitor<'a>(pub &'a Set);

impl<'a, 'de> Visitor<'de> for SetVisitor<'a> {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a set")
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

        self.0
            .ty
            .as_ref(py())
            .call1((items,))
            .map_err(de)
            .map(|v| v.into())
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a Set {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(SetVisitor(self))
    }
}
