use crate::{
    types::{Object, SchemaStack, TypeKind},
    util::*,
};
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple},
};
use serde_state::{
    de::{self, Deserializer, EnumAccess, Error, MapAccess, Seed, SeqAccess, Visitor},
    DeserializeState,
};
use std::fmt;

struct DictVisitor<'a, 'b>(&'b mut SchemaStack<'a>);

impl<'a, 'b, 'de> Visitor<'de> for DictVisitor<'a, 'b> {
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
            self.0.push_by_index(0)?;
            let key = access.next_key_seed(Seed::new(&mut *self.0))?;
            self.0.pop();

            let key: Object = match key {
                Some(key) => key,
                None => {
                    break;
                }
            };

            self.0.push_by_index(1)?;
            let value: Object = access.next_value_seed(Seed::new(&mut *self.0))?;
            self.0.pop();

            args.push((key.to_pyobj(), value.to_pyobj()));
        }

        Ok(self.0.current().call_kw(args)?)
    }
}

struct ClassVisitor<'a, 'b>(&'b mut SchemaStack<'a>);

impl<'a, 'b, 'de> Visitor<'de> for ClassVisitor<'a, 'b> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut args = Vec::new();

        while let Some(key) = access.next_key()? {
            let key: &str = key;

            self.0.push_by_name(key)?;
            let value: Object = access.next_value_seed(Seed::new(&mut *self.0))?;
            self.0.pop();

            args.push((key.into_py(py()), value.to_pyobj()));
        }

        Ok(self.0.current().call_kw(args)?)
    }
}

struct ListVisitor<'a, 'b>(&'b mut SchemaStack<'a>);

impl<'a, 'b, 'de> Visitor<'de> for ListVisitor<'a, 'b> {
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
            self.0.push_by_index(0)?;
            let value = seq.next_element_seed(Seed::new(&mut *self.0))?;
            self.0.pop();

            let value: Object = match value {
                Some(value) => value,
                None => {
                    break;
                }
            };

            items.push(value.to_pyobj());
        }

        Ok(self.0.current().call((items,))?)
    }
}

struct TupleVisitor<'a, 'b>(&'b mut SchemaStack<'a>);

impl<'a, 'b, 'de> Visitor<'de> for TupleVisitor<'a, 'b> {
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
        let len = self.0.current().args.len();

        loop {
            self.0.push_by_index(index.min(len - 1))?;
            let value = seq.next_element_seed(Seed::new(&mut *self.0))?;
            self.0.pop();

            let value: Object = match value {
                Some(value) => value,
                None => {
                    break;
                }
            };

            index += 1;

            items.push(value.to_pyobj());
        }

        Ok(self.0.current().call((items,))?)
    }
}

struct BoolVisitor<'a, 'b>(&'b mut SchemaStack<'a>);

impl<'a, 'b, 'de> Visitor<'de> for BoolVisitor<'a, 'b> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a boolean")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.0.current().call((value,))
    }
}

struct IntVisitor<'a, 'b>(&'b mut SchemaStack<'a>);

impl<'a, 'b, 'de> Visitor<'de> for IntVisitor<'a, 'b> {
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
        self.0.current().call((value,))
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
        self.0.current().call((value,))
    }
}

struct FloatVisitor<'a, 'b>(&'b mut SchemaStack<'a>);

impl<'a, 'b, 'de> Visitor<'de> for FloatVisitor<'a, 'b> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a float")
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.0.current().call((value,))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.0.current().call((value,))
    }
}

struct StrVisitor<'a, 'b>(&'b mut SchemaStack<'a>);

impl<'a, 'b, 'de> Visitor<'de> for StrVisitor<'a, 'b> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a string")
    }

    fn visit_char<E>(self, value: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.0.current().call((value.to_string(),))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.0.current().call((value,))
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.0.current().call((value,))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.0.current().call((value,))
    }
}

struct BytesVisitor<'a, 'b>(&'b mut SchemaStack<'a>);

impl<'a, 'b, 'de> Visitor<'de> for BytesVisitor<'a, 'b> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "byte array")
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.0.current().call((value,))
    }

    fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.0.current().call((value,))
    }

    fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.0.current().call((value,))
    }
}

struct OptionVisitor<'a, 'b>(&'b mut SchemaStack<'a>);

impl<'a, 'b, 'de> Visitor<'de> for OptionVisitor<'a, 'b> {
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
        self.0.push_by_index(0)?;
        let obj = Object::deserialize_state(self.0, deserializer);
        self.0.pop();
        obj
    }
}

impl<'a, 'de> DeserializeState<'de, SchemaStack<'a>> for Object {
    fn deserialize_state<'b, D>(stack: &'b mut SchemaStack<'a>, de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        println!("Looking for {:?}", stack.current().kind);
        match stack.current().kind {
            TypeKind::Bool => de.deserialize_bool(BoolVisitor(stack)),
            TypeKind::Int => de.deserialize_i64(IntVisitor(stack)),
            TypeKind::Float => de.deserialize_i64(FloatVisitor(stack)),
            TypeKind::Str => de.deserialize_str(StrVisitor(stack)),
            TypeKind::Bytes => de.deserialize_bytes(BytesVisitor(stack)),
            TypeKind::List => de.deserialize_seq(ListVisitor(stack)),
            TypeKind::Tuple => de.deserialize_seq(TupleVisitor(stack)),
            TypeKind::Dict => de.deserialize_map(DictVisitor(stack)),
            TypeKind::Class => de.deserialize_map(ClassVisitor(stack)),
            TypeKind::Enum => unimplemented!(),
            TypeKind::Option => de.deserialize_option(OptionVisitor(stack)),
            TypeKind::Union => unimplemented!(),
        }
    }
}
