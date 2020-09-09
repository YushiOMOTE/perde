use crate::{
    types::{self, Object},
    util::*,
};
use serde::{
    de::{Deserializer, EnumAccess, Error, MapAccess, SeqAccess, Visitor},
    Deserialize,
};
use std::{collections::HashMap, fmt};

struct AnyVisitor;

impl<'de> Visitor<'de> for AnyVisitor {
    type Value = Object;

    #[cfg_attr(feature = "perf", flame)]
    fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        types::py_bool(v).map_err(de)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        types::py_i64(v).map_err(de)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        types::py_u64(v).map_err(de)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_f64(v as f64)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        types::py_f64(v).map_err(de)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_borrowed_str(&v.to_string())
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(v)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        types::py_str(v).map_err(de)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_borrowed_str(&v)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_borrowed_bytes(v)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        types::py_bytes(v).map_err(de)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_borrowed_bytes(&v)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        types::py_none().map_err(de)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(AnyVisitor)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        types::py_none().map_err(de)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(AnyVisitor)
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut args: smallvec::SmallVec<[Object; 16]> = smallvec::SmallVec::new();

        while let Some(arg) = seq.next_element()? {
            let arg: Object = arg;
            args.push(arg);
        }

        let mut list = types::List::new(args.len()).map_err(de)?;
        for (i, arg) in args.into_iter().enumerate() {
            list.set(i, arg);
        }

        Ok(list.into_inner())
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut dict = types::Dict::new().map_err(de)?;

        while let Some(k) = map.next_key()? {
            let k: &str = k;
            let v = map.next_value()?;
            dict.set(types::py_str(&k).map_err(de)?, v).map_err(de)?;
        }

        Ok(dict.into_inner())
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        let (v, _) = data.variant()?;
        Ok(v)
    }
}

impl<'de> Deserialize<'de> for Object {
    #[cfg_attr(feature = "perf", flame)]
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_any(AnyVisitor)
    }
}
