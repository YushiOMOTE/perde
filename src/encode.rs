use crate::{
    schema::{Class, Enum, Primitive, Schema, Union},
    types::{DictRef, ListRef, ObjectRef, SetRef, TupleRef},
    util::*,
};
use derive_new::new;
use pyo3::{
    prelude::*,
    types::{PyDict, PyList, PySet, PyTuple},
};
use serde::{
    ser::{self, Error, SerializeMap, SerializeSeq, Serializer},
    Serialize,
};

#[derive(new, Clone, Debug)]
pub struct WithSchema<'a> {
    pub schema: &'a Schema,
    pub object: ObjectRef<'a>,
}

impl<'a> Serialize for WithSchema<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.schema {
            Schema::Primitive(Primitive::Bool) => {
                s.serialize_bool(self.object.as_bool().map_err(ser)?)
            }
            Schema::Primitive(Primitive::Int) => {
                s.serialize_i64(self.object.as_i64().map_err(ser)?)
            }
            Schema::Primitive(Primitive::Str) => {
                s.serialize_str(self.object.as_str().map_err(ser)?)
            }
            Schema::Primitive(Primitive::Float) => {
                s.serialize_f64(self.object.as_f64().map_err(ser)?)
            }
            Schema::Primitive(Primitive::ByteArray) => {
                s.serialize_bytes(self.object.as_bytearray().map_err(ser)?)
            }
            Schema::Primitive(Primitive::Bytes) => {
                s.serialize_bytes(self.object.as_bytes().map_err(ser)?)
            }
            Schema::List(l) => {
                let list = ListRef::new(self.object);
                let len = list.len();
                let mut seq = s.serialize_seq(Some(len))?;
                for i in 0..len {
                    let obj = list.get(i).unwrap();
                    let w = WithSchema::new(&l.value, obj);
                    seq.serialize_element(&w)?;
                }
                seq.end()
            }
            Schema::Set(l) => {
                let set = SetRef::new(self.object);
                let len = set.len();
                let mut seq = s.serialize_seq(Some(len))?;
                for i in 0..len {
                    let obj = set.get(i).unwrap();
                    let w = WithSchema::new(&l.value, obj);
                    seq.serialize_element(&w)?;
                }
                seq.end()
            }
            Schema::Tuple(t) => {
                let tuple = TupleRef::new(self.object);
                let iter = tuple.iter();
                let len = iter.len();
                let mut seq = s.serialize_seq(Some(len))?;
                for (obj, schema) in iter.zip(t.args.iter()) {
                    let w = WithSchema::new(schema, obj);
                    seq.serialize_element(&w)?;
                }
                seq.end()
            }
            Schema::Dict(d) => {
                let dict = DictRef::new(self.object);
                let mut map = s.serialize_map(Some(dict.len()))?;
                for (k, v) in dict.iter() {
                    let k = WithSchema::new(&d.key, k);
                    let v = WithSchema::new(&d.value, v);
                    map.serialize_entry(&k, &v)?;
                }
                map.end()
            }
            Schema::Class(c) => unimplemented!(),
            Schema::Enum(e) => unimplemented!(),
            Schema::Optional(o) => {
                if self.object.is_none() {
                    s.serialize_none()
                } else {
                    let w = WithSchema::new(&o.value, self.object);
                    s.serialize_some(&w)
                }
            }
            Schema::Union(u) => unimplemented!(),
            Schema::Any(a) => unimplemented!(),
        }
    }
}
