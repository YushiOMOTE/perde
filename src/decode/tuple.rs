use crate::{
    error::Convert,
    schema::*,
    types::{self, Object},
};
use pyo3::{prelude::*, types::PyTuple};
use serde::de::{DeserializeSeed, Deserializer, IgnoredAny, SeqAccess, Visitor};
use smallvec::SmallVec;
use std::fmt;

pub struct TupleVisitor<'a>(pub &'a Tuple);

impl<'a, 'de> Visitor<'de> for TupleVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a tuple")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut items = SmallVec::<[_; 16]>::new();

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

        let mut tuple = types::Tuple::new(items.len()).de()?;
        for (i, a) in items.into_iter().enumerate() {
            tuple.set(i, a);
        }

        Ok(tuple.into_inner())
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a Tuple {
    type Value = Object;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(TupleVisitor(self))
    }
}
