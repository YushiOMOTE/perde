use crate::{
    types::{Object, SchemaStack, TypeKind},
    util::*,
};
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple},
};
use serde_state::{
    de::{self, Deserializer, Error, MapAccess, Seed, SeqAccess, Visitor},
    DeserializeState,
};
use std::fmt;

struct ObjectVisitor<'a, 'b>(&'b mut SchemaStack<'a>);

impl<'a, 'b> ObjectVisitor<'a, 'b> {
    fn value<E: de::Error>(&self, args: impl IntoPy<Py<PyTuple>>) -> Result<Object, E> {
        self.0.current().call(args, None).map_err(de)
    }
}

impl<'a, 'b, 'de> Visitor<'de> for ObjectVisitor<'a, 'b> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unexpected format")
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
        self.value((value,))
    }

    fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.value((value,))
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
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
        self.value((value,))
    }

    fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.value((value,))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.value((value,))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.value((value,))
    }

    fn visit_char<E>(self, value: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.value((value as u32,))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.value((value,))
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.value((value,))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.value((value,))
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.value((value,))
    }

    fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.value((value,))
    }

    fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.value((value,))
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
        Object::deserialize_state(self.0, deserializer)
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
        Object::deserialize_state(self.0, deserializer)
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let dict = PyDict::new(py());

        match &self.0.current().kind {
            TypeKind::Dict => loop {
                self.0.push_by_index(0).map_err(de)?;
                let key: Object = match access.next_key_seed(Seed::new(&mut *self.0))? {
                    Some(key) => key,
                    None => {
                        self.0.pop();
                        break;
                    }
                };
                self.0.pop();

                self.0.push_by_index(1).map_err(de)?;
                let value: Object = access.next_value_seed(Seed::new(&mut *self.0))?;
                self.0.pop();

                dict.set_item(key.to_pyobj(), value.to_pyobj())
                    .map_err(de)?;
            },
            TypeKind::Class => {
                while let Some(key) = access.next_key()? {
                    let key: String = key;

                    self.0.push_by_name(&key).map_err(de)?;
                    let value: Object = access.next_value_seed(Seed::new(&mut *self.0))?;
                    self.0.pop();

                    dict.set_item(key, value.to_pyobj()).map_err(de)?;
                }
            }
            kind => unreachable!("The type kind must be dict or class; got {:?}", kind),
        }

        Ok(self.0.current().call((), Some(dict)).map_err(de)?)
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut items = Vec::new();

        match &self.0.current().kind {
            TypeKind::List => loop {
                self.0.push_by_index(0).map_err(de)?;
                let value: Object = match seq.next_element_seed(Seed::new(&mut *self.0))? {
                    Some(value) => value,
                    None => {
                        self.0.pop();
                        break;
                    }
                };
                self.0.pop();

                items.push(value.to_pyobj());
            },
            TypeKind::Tuple => {
                let mut index = 0;
                let len = self.0.current().args.len();

                loop {
                    self.0.push_by_index(index.min(len - 1)).map_err(de)?;
                    let value: Object = match seq.next_element_seed(Seed::new(&mut *self.0))? {
                        Some(value) => value,
                        None => {
                            self.0.pop();
                            break;
                        }
                    };
                    self.0.pop();

                    index += 1;

                    items.push(value.to_pyobj());
                }
            }
            kind => unreachable!("The type kind must be list or tuple; got {:?}", kind),
        }

        Ok(self.0.current().call((items,), None).map_err(de)?)
    }
}

impl<'a, 'de> DeserializeState<'de, SchemaStack<'a>> for Object {
    fn deserialize_state<'b, D>(stack: &'b mut SchemaStack<'a>, de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match stack.current().kind {
            TypeKind::Bool => de.deserialize_bool(ObjectVisitor(stack)),
            TypeKind::Int => de.deserialize_i64(ObjectVisitor(stack)),
            TypeKind::Str => de.deserialize_str(ObjectVisitor(stack)),
            TypeKind::Bytes => de.deserialize_bytes(ObjectVisitor(stack)),
            TypeKind::List => de.deserialize_seq(ObjectVisitor(stack)),
            TypeKind::Tuple => de.deserialize_seq(ObjectVisitor(stack)),
            TypeKind::Dict => de.deserialize_map(ObjectVisitor(stack)),
            TypeKind::Class => de.deserialize_map(ObjectVisitor(stack)),
            TypeKind::Enum => unimplemented!(),
            TypeKind::Option => de.deserialize_option(ObjectVisitor(stack)),
            TypeKind::Union => unimplemented!(),
        }
    }
}
