use crate::{
    state::{DeserializeState, Seed},
    types::{Object, Schema, TypeKind},
    util::*,
};
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple},
};
use serde::de::{
    self, Deserialize, Deserializer, EnumAccess, Error, IgnoredAny, MapAccess, SeqAccess,
    Unexpected, Visitor,
};
use std::{collections::HashMap, fmt};

struct DictVisitor<'a>(&'a Schema);

impl<'a, 'de> Visitor<'de> for DictVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut args = Vec::new();

        loop {
            let seed = Seed::new(self.0.type_param(0)?);
            let key = access.next_key_seed(seed)?;

            let key: Object = match key {
                Some(key) => key,
                None => {
                    break;
                }
            };

            let seed = Seed::new(self.0.type_param(1)?);
            let value: Object = access.next_value_seed(seed)?;

            args.push((key.to_pyobj(), value.to_pyobj()));
        }

        Ok(self.0.call_map(args)?)
    }
}

struct ClassVisitor<'a>(&'a Schema);

impl<'a, 'de> Visitor<'de> for ClassVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut args = HashMap::new();

        while let Some(key) = access.next_key()? {
            let key: &str = key;

            if let Some(schema) = self.0.member(key)? {
                let seed = Seed::new(schema);
                let value: Object = access.next_value_seed(seed)?;

                args.insert(key, value.to_pyobj());
            } else {
                let _: IgnoredAny = access.next_value()?;
            }
        }

        let v = if self.0.has_flatten() {
            self.0.call_flatten(&mut args)?
        } else {
            self.0.call_class(&mut args)?
        };

        Ok(v)
    }
}

struct ListVisitor<'a>(&'a Schema);

impl<'a, 'de> Visitor<'de> for ListVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut items = Vec::new();

        loop {
            let seed = Seed::new(self.0.type_param(0)?);
            let value: Object = match seq.next_element_seed(seed)? {
                Some(value) => value,
                None => break,
            };

            items.push(value.to_pyobj());
        }

        Ok(self.0.call((items,))?)
    }
}

struct TupleVisitor<'a>(&'a Schema);

impl<'a, 'de> Visitor<'de> for TupleVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a sequence")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut items = Vec::new();

        let mut index = 0;
        let len = self.0.args.len();

        loop {
            let seed = Seed::new(self.0.type_param(index.min(len.saturating_sub(1)))?);
            let value: Object = match seq.next_element_seed(seed)? {
                Some(value) => {
                    if index >= len {
                        return Err(Error::custom(format!(
                            "the tuple expects {} elements but got more",
                            len
                        )));
                    } else {
                        value
                    }
                }
                None => break,
            };

            index += 1;
            items.push(value.to_pyobj());
        }

        Ok(self.0.call((items,))?)
    }
}

struct BoolVisitor<'a>(&'a Schema);

impl<'a, 'de> Visitor<'de> for BoolVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a boolean")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Object::new(value))
    }
}

struct IntVisitor<'a>(&'a Schema);

impl<'a, 'de> Visitor<'de> for IntVisitor<'a> {
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
        Ok(Object::new(value))
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
        Ok(Object::new(value))
    }
}

struct FloatVisitor<'a>(&'a Schema);

impl<'a, 'de> Visitor<'de> for FloatVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a float")
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_f64(value as f64)
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(value))
    }
}

struct StrVisitor<'a>(&'a Schema);

impl<'a, 'de> Visitor<'de> for StrVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a string")
    }

    fn visit_char<E>(self, value: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(value.to_string()))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(value))
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(value))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(value))
    }
}

struct BytesVisitor<'a>(&'a Schema);

impl<'a, 'de> Visitor<'de> for BytesVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "byte array")
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.0.call((value,))
    }

    fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.0.call((value,))
    }

    fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.0.call((value,))
    }
}

struct OptionVisitor<'a>(&'a Schema);

impl<'a, 'de> Visitor<'de> for OptionVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an option")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::null())
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let schema = self.0.type_param(0)?;
        Object::deserialize_state(schema, deserializer)
    }
}

struct UnionVisitor<'a>(&'a Schema);

impl<'a, 'c> UnionVisitor<'a> {
    fn find_container<E>(
        &mut self,
        kind: &[TypeKind],
        unexpected: Unexpected<'c>,
    ) -> Result<&Schema, E>
    where
        E: Error,
    {
        self.0
            .args
            .iter()
            .find(|s| kind.contains(&s.kind))
            .ok_or_else(|| Error::invalid_type(unexpected, self))
    }

    fn find<E>(&mut self, kind: TypeKind, unexpected: Unexpected<'c>) -> Result<&Schema, E>
    where
        E: Error,
    {
        self.0
            .args
            .iter()
            .find(|s| s.kind == kind)
            .ok_or_else(|| Error::invalid_type(unexpected, self))
    }
}

impl<'a, 'de> Visitor<'de> for UnionVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let names: Vec<_> = self
            .0
            .args
            .iter()
            .map(|s| s.cls.as_ref(py()).name())
            .collect();
        write!(f, "any of {:?}", names)
    }

    fn visit_bool<E>(mut self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let schema = self.find(TypeKind::Bool, Unexpected::Bool(v))?;
        BoolVisitor(schema).visit_bool(v)
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_i64(v as i64)
    }

    fn visit_i64<E>(mut self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let schema = self.find(TypeKind::Int, Unexpected::Signed(v))?;
        IntVisitor(schema).visit_i64(v)
    }

    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_u64(v as u64)
    }

    fn visit_u64<E>(mut self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let schema = self.find(TypeKind::Int, Unexpected::Unsigned(v))?;
        IntVisitor(schema).visit_u64(v)
    }

    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_f64(v as f64)
    }

    fn visit_f64<E>(mut self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let schema = self.find(TypeKind::Float, Unexpected::Float(v))?;
        FloatVisitor(schema).visit_f64(v)
    }

    fn visit_char<E>(mut self, v: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let schema = self.find(TypeKind::Str, Unexpected::Str(&v.to_string()))?;
        StrVisitor(schema).visit_char(v)
    }

    fn visit_str<E>(mut self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let schema = self.find(TypeKind::Str, Unexpected::Str(v))?;
        StrVisitor(schema).visit_str(v)
    }

    fn visit_borrowed_str<E>(mut self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let schema = self.find(TypeKind::Str, Unexpected::Str(v))?;
        StrVisitor(schema).visit_borrowed_str(v)
    }

    fn visit_string<E>(mut self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let schema = self.find(TypeKind::Str, Unexpected::Str(&v))?;
        StrVisitor(schema).visit_string(v)
    }

    fn visit_bytes<E>(mut self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let schema = self.find(TypeKind::Bytes, Unexpected::Bytes(v))?;
        BytesVisitor(schema).visit_bytes(v)
    }

    fn visit_borrowed_bytes<E>(mut self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let schema = self.find(TypeKind::Bytes, Unexpected::Bytes(v))?;
        BytesVisitor(schema).visit_borrowed_bytes(v)
    }

    fn visit_byte_buf<E>(mut self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let schema = self.find(TypeKind::Bytes, Unexpected::Bytes(&v))?;
        BytesVisitor(schema).visit_byte_buf(v)
    }

    fn visit_none<E>(mut self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let schema = self.find(TypeKind::Option, Unexpected::Option)?;
        OptionVisitor(schema).visit_none()
    }

    fn visit_some<D>(mut self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let schema = self.find(TypeKind::Option, Unexpected::Option)?;
        OptionVisitor(schema).visit_some(deserializer)
    }

    fn visit_seq<A>(mut self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let schema = self.find_container(&[TypeKind::List, TypeKind::Tuple], Unexpected::Seq)?;

        match schema.kind {
            TypeKind::List => ListVisitor(schema).visit_seq(seq),
            TypeKind::Tuple => TupleVisitor(schema).visit_seq(seq),
            _ => unreachable!(),
        }
    }

    fn visit_map<A>(mut self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let schema = self.find_container(&[TypeKind::Dict, TypeKind::Class], Unexpected::Seq)?;

        match schema.kind {
            TypeKind::Dict => DictVisitor(schema).visit_map(map),
            TypeKind::Class => ClassVisitor(schema).visit_map(map),
            _ => unreachable!(),
        }
    }
}

struct EnumVisitor<'a>(&'a Schema);

impl<'a> EnumVisitor<'a> {
    fn vars(&self) -> Vec<&str> {
        self.0
            .kwargs
            .iter()
            .map(|(name, _)| name.as_ref())
            .collect()
    }

    fn get<E>(&self, s: &str) -> Result<Object, E>
    where
        E: Error,
    {
        match self.0.kwargs.iter().find(|(name, _)| *name == s) {
            Some(_) => Ok(self
                .0
                .cls
                .as_ref(py())
                .getattr(s)
                .map(|v| Object::new(v))
                .map_err(de)?),
            None => Err(Error::custom(format!(
                "the enum value must be any of {:?}",
                self.vars()
            ))),
        }
    }
}

impl<'a, 'de> Visitor<'de> for EnumVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a enum value: {:?}", self.vars())
    }

    fn visit_char<E>(self, value: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.get(&value.to_string())
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.get(value)
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.get(value)
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.get(&value)
    }
}

struct AnyVisitor;

impl<'de> Visitor<'de> for AnyVisitor {
    type Value = Object;

    fn expecting(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }

    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }
    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }
    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }
    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }
    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }
    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v.to_string()))
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }
    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }
    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(v))
    }
    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::null())
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(AnyVisitor)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::null())
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(AnyVisitor)
    }

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

    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        let (v, _) = data.variant()?;
        Ok(v)
    }
}

impl<'a, 'de> DeserializeState<'de, Schema> for Object {
    fn deserialize_state<'b, D>(schema: &Schema, de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match schema.kind {
            TypeKind::Bool => de.deserialize_bool(BoolVisitor(schema)),
            TypeKind::Int => de.deserialize_i64(IntVisitor(schema)),
            TypeKind::Float => de.deserialize_i64(FloatVisitor(schema)),
            TypeKind::Str => de.deserialize_str(StrVisitor(schema)),
            TypeKind::Bytes => de.deserialize_bytes(BytesVisitor(schema)),
            TypeKind::List => de.deserialize_seq(ListVisitor(schema)),
            TypeKind::Tuple => de.deserialize_seq(TupleVisitor(schema)),
            TypeKind::Dict => de.deserialize_map(DictVisitor(schema)),
            TypeKind::Class => de.deserialize_map(ClassVisitor(schema)),
            TypeKind::Enum => de.deserialize_any(EnumVisitor(schema)),
            TypeKind::Option => de.deserialize_option(OptionVisitor(schema)),
            TypeKind::Union => de.deserialize_any(UnionVisitor(schema)),
        }
    }
}

impl<'de> Deserialize<'de> for Object {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        de.deserialize_any(AnyVisitor)
    }
}
