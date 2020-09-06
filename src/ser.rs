use crate::{
    types::{Object, Schema, TypeKind},
    util::*,
};
use pyo3::{
    prelude::*,
    types::{PyDict, PyList, PyTuple},
};
use serde::{
    ser::{SerializeMap, SerializeSeq, Serializer},
    Serialize,
};

pub struct TypedObject<'a> {
    schema: &'a Schema,
    object: Object,
}

impl<'a> TypedObject<'a> {
    pub fn new<T: ToPyObject>(schema: &'a Schema, object: T) -> Self {
        Self {
            schema,
            object: Object::new(object),
        }
    }
}

impl<'a> Serialize for TypedObject<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.schema.kind() {
            TypeKind::Bool => {
                let v = self.object.to_value()?;
                s.serialize_bool(v)
            }
            TypeKind::Int => {
                let v = self.object.to_value()?;
                s.serialize_i64(v)
            }
            TypeKind::Float => {
                let v = self.object.to_value()?;
                s.serialize_f64(v)
            }
            TypeKind::Str => {
                let v = self.object.to_value()?;
                s.serialize_str(v)
            }
            TypeKind::Bytes => {
                let v = self.object.to_value()?;
                s.serialize_bytes(v)
            }
            TypeKind::ByteArray => {
                let v: Vec<u8> = self.object.to_value()?;
                s.serialize_bytes(&v)
            }
            TypeKind::List => {
                let list: &PyList = self.object.to_value()?;
                let mut seq = s.serialize_seq(Some(list.len()))?;
                let schema = self.schema.type_param(0).map_err(ser)?;
                for v in list {
                    let object = TypedObject::new(schema, v);
                    seq.serialize_element(&object)?;
                }
                seq.end()
            }
            TypeKind::Tuple => {
                let list: &PyTuple = self.object.to_value()?;
                let mut seq = s.serialize_seq(Some(list.len()))?;

                for (i, v) in list.into_iter().enumerate() {
                    let schema = self.schema.type_param(i).map_err(ser)?;
                    let object = TypedObject::new(schema, v);
                    seq.serialize_element(&object)?;
                }
                seq.end()
            }
            TypeKind::Dict => {
                let dict: &PyDict = self.object.to_value()?;
                let mut map = s.serialize_map(Some(dict.len()))?;
                let ks = self.schema.type_param(0).map_err(ser)?;
                let vs = self.schema.type_param(1).map_err(ser)?;
                for (k, v) in dict {
                    let k = TypedObject::new(ks, k);
                    let v = TypedObject::new(vs, v);
                    map.serialize_entry(&k, &v)?;
                }
                map.end()
            }
            TypeKind::Class => {
                let v: &PyAny = self.object.to_value()?;
                let mems = self.schema.retrieve_members(v)?;
                let mut map = s.serialize_map(Some(mems.len()))?;
                for (k, v, s) in mems {
                    let v = TypedObject::new(s, v);
                    map.serialize_entry(&k, &v)?;
                }
                map.end()
            }
            TypeKind::Enum => {
                let v: &PyAny = self.object.to_value()?;
                let name = self.schema.verify_variant(v)?;
                name.serialize(s)
            }
            TypeKind::Option => {
                let some: Option<&PyAny> = self.object.to_value()?;
                match some {
                    Some(v) => {
                        let schema = self.schema.type_param(0).map_err(ser)?;
                        let v = TypedObject::new(schema, v);
                        s.serialize_some(&v)
                    }
                    None => s.serialize_none(),
                }
            }
            TypeKind::Union => {
                let v: &PyAny = self.object.to_value()?;
                let schema = self.schema.find_union_variant(v)?;
                let v = TypedObject::new(schema, v);
                v.serialize(s)
            }
        }
    }
}
