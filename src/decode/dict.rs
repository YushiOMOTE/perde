use crate::{
    error::Convert,
    schema::*,
    types::{self, Object},
};
use pyo3::{prelude::*, types::PyDict};
use serde::de::{DeserializeSeed, Deserializer, MapAccess, Visitor};
use std::fmt;

pub struct DictVisitor<'a>(pub &'a Dict);

impl<'a, 'de> Visitor<'de> for DictVisitor<'a> {
    type Value = Object;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a map")
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut dict = types::Dict::new().de()?;

        while let Some(key) = access.next_key_seed(&*self.0.key)? {
            let key: Object = key;
            let value: Object = access.next_value_seed(&*self.0.value)?;
            dict.set(key, value);
        }

        Ok(dict.into_inner())
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a Dict {
    type Value = Object;

    #[cfg_attr(feature = "perf", flame)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(DictVisitor(self))
    }
}
