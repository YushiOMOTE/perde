use crate::{
    schema::*,
    types::{self, Object},
    util::*,
};
use pyo3::prelude::*;
use serde::de::{DeserializeSeed, Deserializer, SeqAccess, Visitor};
use std::fmt;

pub struct SetVisitor<'a>(pub &'a Set);

impl<'a, 'de> Visitor<'de> for SetVisitor<'a> {
    type Value = Object;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a set")
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut set = types::Set::new().map_err(de)?;

        while let Some(value) = seq.next_element_seed(&*self.0.value)? {
            set.set(value).map_err(de)?;
        }

        Ok(set.into_inner())
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a Set {
    type Value = Object;

    #[cfg_attr(feature = "perf", flame)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(SetVisitor(self))
    }
}
