use crate::{schema::*, util::*};
use pyo3::prelude::*;
use serde::de::{self, DeserializeSeed, Deserializer, SeqAccess, Visitor};
use std::fmt;

pub struct BoolVisitor;

impl<'de> Visitor<'de> for BoolVisitor {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a boolean")
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value.to_object(py()))
    }
}

pub struct IntVisitor;

impl<'de> Visitor<'de> for IntVisitor {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an integer")
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(value as i64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(value as i64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value.to_object(py()))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u64(value as u64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u64(value as u64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u64(value as u64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value.to_object(py()))
    }
}

pub struct FloatVisitor;

impl<'de> Visitor<'de> for FloatVisitor {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a float")
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_f64(value as f64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value.to_object(py()))
    }
}

pub struct StrVisitor;

impl<'de> Visitor<'de> for StrVisitor {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a string")
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_char<E>(self, value: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&value.to_string())
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_str(value)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(value.to_object(py()))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&value)
    }
}

pub struct BytesVisitor<'a>(pub &'a Bytes);

impl<'a, 'de> Visitor<'de> for BytesVisitor<'a> {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bytes")
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_bytes(value)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.0
            .ty
            .as_ref(py())
            .call1((value,))
            .map_err(de)
            .map(|v| v.into())
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_bytes(&value)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut bytes = Vec::<u8>::new();

        loop {
            bytes.push(match seq.next_element()? {
                Some(value) => value,
                None => break,
            });
        }

        self.visit_borrowed_bytes(&bytes)
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a Primitive {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self {
            Primitive::Bool => deserializer.deserialize_bool(BoolVisitor),
            Primitive::Int => deserializer.deserialize_i64(IntVisitor),
            Primitive::Float => deserializer.deserialize_f64(FloatVisitor),
            Primitive::Str => deserializer.deserialize_str(StrVisitor),
            Primitive::Bytes(bytes) => deserializer.deserialize_bytes(BytesVisitor(&bytes)),
        }
    }
}
