use crate::{decode::primitive::*, error::Convert, object::Object, schema::*};
use serde::de::{DeserializeSeed, Deserializer};

pub mod any;
pub mod class;
pub mod dict;
pub mod enums;
pub mod frozen_set;
pub mod list;
pub mod primitive;
pub mod set;
pub mod tuple;
pub mod union;

impl<'a, 'de> DeserializeSeed<'de> for &'a Schema {
    type Value = Object;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        match self {
            Schema::Bool => deserializer.deserialize_bool(BoolVisitor),
            Schema::Int => deserializer.deserialize_i64(IntVisitor),
            Schema::Float => deserializer.deserialize_f64(FloatVisitor),
            Schema::Str => deserializer.deserialize_str(StrVisitor),
            Schema::Bytes => deserializer.deserialize_bytes(BytesVisitor(false)),
            Schema::ByteArray => deserializer.deserialize_bytes(BytesVisitor(true)),
            Schema::DateTime => {
                let s = deserializer.deserialize_str(StrVisitor)?;
                s.into_datetime().de()
            }
            Schema::Date => {
                let s = deserializer.deserialize_str(StrVisitor)?;
                s.into_date().de()
            }
            Schema::Time => {
                let s = deserializer.deserialize_str(StrVisitor)?;
                s.into_time().de()
            }
            Schema::Decimal => {
                let s = deserializer.deserialize_str(StrVisitor)?;
                s.into_decimal().de()
            }
            Schema::Uuid => {
                let s = deserializer.deserialize_str(StrVisitor)?;
                s.into_uuid().de()
            }
            Schema::Dict(d) => d.deserialize(deserializer),
            Schema::List(l) => l.deserialize(deserializer),
            Schema::Set(s) => s.deserialize(deserializer),
            Schema::FrozenSet(s) => s.deserialize(deserializer),
            Schema::Tuple(t) => t.deserialize(deserializer),
            Schema::Class(c) => c.deserialize(deserializer),
            Schema::Enum(e) => e.deserialize(deserializer),
            Schema::Union(u) => u.deserialize(deserializer),
            Schema::Any(a) => a.deserialize(deserializer),
        }
    }
}
