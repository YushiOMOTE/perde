use crate::{schema::*, util::*};
use pyo3::{prelude::*, types::PyTuple};
use serde::de::{DeserializeSeed, Deserializer, IgnoredAny, SeqAccess, Visitor};
use std::fmt;

pub struct TupleVisitor<'a>(pub &'a Tuple);

impl<'a, 'de> Visitor<'de> for TupleVisitor<'a> {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a tuple")
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut items = vec![];

        let mut args = self.0.args.iter().fuse();

        loop {
            if let Some(schema) = args.next() {
                match seq.next_element_seed(schema)? {
                    Some(value) => items.push(value),
                    None => break,
                }
            } else {
                let _: IgnoredAny = match seq.next_element()? {
                    Some(value) => value,
                    None => break,
                };
            }
        }

        Ok(PyTuple::new(py(), items).into())
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a Tuple {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(TupleVisitor(self))
    }
}
