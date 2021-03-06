use crate::{decode, object::Object, schema::*};
use serde::de::{
    self, DeserializeSeed, Deserializer, IntoDeserializer, MapAccess, SeqAccess, Unexpected,
    Visitor,
};
use std::fmt;

struct UnionVisitor<'a>(&'a Union);

macro_rules! find {
    ($s:expr, $unx:expr, $($kind:tt),*) => {
        $s.0.variants.iter().find(|s| matches!(s, $(Schema::$kind(_))|*))
        .ok_or_else(|| de::Error::invalid_type($unx, &$s))
    }
}

macro_rules! find_p {
    ($s:expr, $unx:expr, $($kind:tt),*) => {
        $s.0.variants.iter().find(|s| matches!(s, $(Schema::$kind)|*))
            .ok_or_else(|| de::Error::invalid_type($unx, &$s))
    }
}

impl<'a, 'de> Visitor<'de> for UnionVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let names: Vec<_> = self.0.variants.iter().map(|v| v.name()).collect();
        if self.0.optional {
            write!(f, "any of {:?} or None", names)
        } else {
            write!(f, "any of {:?}", names)
        }
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let schema = find_p!(self, Unexpected::Bool(v), Bool)?;
        schema.deserialize(v.into_deserializer())
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let schema = find_p!(self, Unexpected::Signed(v), Int)?;
        schema.deserialize(v.into_deserializer())
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u64(v as u64)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u64(v as u64)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_u64(v as u64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let schema = find_p!(self, Unexpected::Unsigned(v), Int)?;
        schema.deserialize(v.into_deserializer())
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_f64(v as f64)
    }

    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let schema = find_p!(self, Unexpected::Float(v), Float)?;
        schema.deserialize(v.into_deserializer())
    }

    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_str(&v.to_string())
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_str(v)
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let schema = find_p!(self, Unexpected::Str(v), Str)?;
        schema.deserialize(v.into_deserializer())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_str(&v)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_bytes(v)
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let schema = find_p!(self, Unexpected::Bytes(v), Bytes, ByteArray)?;

        match schema {
            Schema::Bytes => decode::primitive::BytesVisitor(false).visit_borrowed_bytes(v),
            Schema::ByteArray => decode::primitive::BytesVisitor(true).visit_borrowed_bytes(v),
            _ => Err(de::Error::invalid_type(Unexpected::Bytes(v), &self)),
        }
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_borrowed_bytes(&v)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if self.0.optional {
            Ok(Object::new_none())
        } else {
            Err(de::Error::invalid_type(Unexpected::Unit, &self))
        }
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if self.0.optional {
            return Ok(Object::new_none());
        }

        let schema = find!(self, Unexpected::Option, Union)?;

        match schema {
            Schema::Union(u) if u.optional => Ok(Object::new_none()),
            _ => Err(de::Error::invalid_type(Unexpected::Option, &self)),
        }
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        find!(self, Unexpected::Option, Union)?;
        deserializer.deserialize_any(UnionVisitor(self.0))
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let schema = self
            .0
            .variants
            .iter()
            .find(|s| matches!(s, Schema::Bytes | Schema::ByteArray | Schema::List(_) | Schema::Tuple(_) | Schema::Set(_)))
            .ok_or_else(|| de::Error::invalid_type(Unexpected::Seq, &self))?;

        match schema {
            Schema::Bytes => decode::primitive::BytesVisitor(false).visit_seq(seq),
            Schema::ByteArray => decode::primitive::BytesVisitor(true).visit_seq(seq),
            Schema::List(l) => decode::list::ListVisitor(l).visit_seq(seq),
            Schema::Tuple(t) => decode::tuple::TupleVisitor(t).visit_seq(seq),
            Schema::Set(s) => decode::set::SetVisitor(s).visit_seq(seq),
            _ => unreachable!(),
        }
    }

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
    type Value = Object;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(UnionVisitor(self))
    }
}
