use crate::{decode, schema::*};
use pyo3::prelude::*;
use serde::de::{
    self, DeserializeSeed, Deserializer, IntoDeserializer, MapAccess, SeqAccess, Unexpected,
    Visitor,
};
use std::fmt;

struct UnionVisitor<'a>(&'a Union);

macro_rules! find {
    ($s:expr, $unx:expr, $($kind:tt),*) => {
        $s.0.variants.iter().find(|s| match s {
            $(Schema::$kind(_) => true,)*
                _ => false,
        })
        .ok_or_else(|| de::Error::invalid_type($unx, &$s))
    }
}

impl<'a, 'de> Visitor<'de> for UnionVisitor<'a> {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let names: Vec<_> = self.0.variants.iter().map(|v| v.name()).collect();
        write!(f, "any of {:?}", names)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let schema = find!(self, Unexpected::Bool(v), Primitive)?;
        schema.deserialize(v.into_deserializer())
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(v as i64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(v as i64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(v as i64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let schema = find!(self, Unexpected::Signed(v), Primitive)?;
        schema.deserialize(v.into_deserializer())
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u64(v as u64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u64(v as u64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u64(v as u64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let schema = find!(self, Unexpected::Unsigned(v), Primitive)?;
        schema.deserialize(v.into_deserializer())
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_f64(v as f64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let schema = find!(self, Unexpected::Float(v), Primitive)?;
        schema.deserialize(v.into_deserializer())
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_str(&v.to_string())
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_str(v)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let schema = find!(self, Unexpected::Str(v), Primitive)?;
        schema.deserialize(v.into_deserializer())
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_str(&v)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_bytes(v)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let schema = find!(self, Unexpected::Bytes(v), Primitive)?;

        match schema {
            Schema::Primitive(Primitive::Bytes(bytes)) => {
                decode::primitive::BytesVisitor(bytes).visit_borrowed_bytes(v)
            }
            _ => Err(de::Error::invalid_type(Unexpected::Bytes(v), &self)),
        }
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_bytes(&v)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let schema = find!(self, Unexpected::Option, Optional)?;

        match schema {
            Schema::Optional(option) => decode::option::OptionVisitor(option).visit_none(),
            _ => Err(de::Error::invalid_type(Unexpected::Option, &self)),
        }
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let schema = find!(self, Unexpected::Option, Optional)?;
        schema.deserialize(deserializer)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let schema = find!(self, Unexpected::Seq, List, Tuple, Set)?;

        match schema {
            Schema::List(l) => decode::list::ListVisitor(l).visit_seq(seq),
            Schema::Tuple(t) => decode::tuple::TupleVisitor(t).visit_seq(seq),
            Schema::Set(s) => decode::set::SetVisitor(s).visit_seq(seq),
            _ => unreachable!(),
        }
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let schema = find!(self, Unexpected::Map, Dict, Class)?;

        match schema {
            Schema::Dict(d) => decode::dict::DictVisitor(d).visit_map(map),
            Schema::Class(c) => decode::class::ClassVisitor(c).visit_map(map),
            _ => unreachable!(),
        }
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a Union {
    type Value = PyObject;

    #[cfg_attr(feature = "perf", flame)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(UnionVisitor(self))
    }
}
