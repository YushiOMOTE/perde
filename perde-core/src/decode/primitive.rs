use crate::{
    error::Convert,
    schema::*,
    types::{
        date_fromisoformat, datetime_fromisoformat, time_fromisoformat, to_decimal, to_uuid, Object,
    },
};
use serde::de::{self, DeserializeSeed, Deserializer, SeqAccess, Visitor};
use smallvec::SmallVec;
use std::fmt;

pub struct BoolVisitor;

impl<'de> Visitor<'de> for BoolVisitor {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a boolean")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Object::new_bool(value))
    }
}

pub struct IntVisitor;

impl<'de> Visitor<'de> for IntVisitor {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an integer")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(value as i64)
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(value as i64)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Object::new_i64(value).de()
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u64(value as u64)
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u64(value as u64)
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u64(value as u64)
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Object::new_u64(value).de()
    }
}

pub struct FloatVisitor;

impl<'de> Visitor<'de> for FloatVisitor {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a float")
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_f64(value as f64)
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Object::new_f64(value).de()
    }
}

pub struct StrVisitor;

impl<'de> Visitor<'de> for StrVisitor {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a string")
    }

    fn visit_char<E>(self, value: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&value.to_string())
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_str(value)
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Object::new_str(value).de()
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&value)
    }
}

pub struct BytesVisitor(pub bool);

impl<'de> Visitor<'de> for BytesVisitor {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bytes")
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_bytes(value)
    }

    fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if self.0 {
            Object::new_bytearray(value).de()
        } else {
            Object::new_bytes(value).de()
        }
    }

    fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_bytes(&value)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut bytes = SmallVec::<[_; 64]>::new();

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
    type Value = Object;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self {
            Primitive::Bool => deserializer.deserialize_bool(BoolVisitor),
            Primitive::Int => deserializer.deserialize_i64(IntVisitor),
            Primitive::Float => deserializer.deserialize_f64(FloatVisitor),
            Primitive::Str => deserializer.deserialize_str(StrVisitor),
            Primitive::Bytes => deserializer.deserialize_bytes(BytesVisitor(false)),
            Primitive::ByteArray => deserializer.deserialize_bytes(BytesVisitor(true)),
            Primitive::DateTime => {
                let s = deserializer.deserialize_str(StrVisitor)?;
                datetime_fromisoformat(&s).de()
            }
            Primitive::Date => {
                let s = deserializer.deserialize_str(StrVisitor)?;
                date_fromisoformat(&s).de()
            }
            Primitive::Time => {
                let s = deserializer.deserialize_str(StrVisitor)?;
                time_fromisoformat(&s).de()
            }
            Primitive::Decimal => {
                let s = deserializer.deserialize_str(StrVisitor)?;
                to_decimal(&s).de()
            }
            Primitive::Uuid => {
                let s = deserializer.deserialize_str(StrVisitor)?;
                to_uuid(&s).de()
            }
        }
    }
}
