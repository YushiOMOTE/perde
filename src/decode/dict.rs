use crate::{schema::*, util::*};
use pyo3::{prelude::*, types::PyDict};
use serde::de::{self, DeserializeSeed, Deserializer, MapAccess, Visitor};
use std::{collections::HashMap, fmt};

pub struct DictVisitor<'a>(pub &'a Dict);

impl<'a, 'de> Visitor<'de> for DictVisitor<'a> {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a map")
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut dict = vec![];

        while let Some(key) = access.next_key_seed(&*self.0.key)? {
            let key: PyObject = key;
            let value: PyObject = access.next_value_seed(&*self.0.value)?;
            dict.push((key, value));
        }

        PyDict::from_sequence(py(), dict.to_object(py()))
            .map_err(de)
            .map(|v| v.into())
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a Dict {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(DictVisitor(self))
    }
}
