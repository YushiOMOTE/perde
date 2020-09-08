use crate::object::Object;
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
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v.to_string()))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::null())
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
        Ok(Object::null())
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
        let mut args = Vec::new();

        while let Some(arg) = seq.next_element()? {
            let arg: Object = arg;
            args.push(arg.to_pyobj());
        }

        Ok(Object::new(args))
    }

    #[cfg_attr(feature = "perf", flame)]
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut args = HashMap::new();

        while let Some(k) = map.next_key()? {
            let k: String = k;
            let v: Object = map.next_value()?;
            args.insert(k, v.to_pyobj());
        }

        Ok(Object::new(args))
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
