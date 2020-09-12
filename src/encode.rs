use crate::{
    object::TypedObject,
    schema::{Class, Enum, Primitive, Schema, Union},
    util::*,
};
use pyo3::{
    prelude::*,
    types::{PyDict, PyList, PySet, PyTuple},
};
use serde::{
    ser::{self, Error, SerializeMap, SerializeSeq, Serializer},
    Serialize,
};

impl<'a> Serialize for TypedObject<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.schema {
            Schema::Primitive(p) => match p {
                Primitive::Bool => {
                    let v = self.object.extract().map_err(ser)?;
                    s.serialize_bool(v)
                }
                Primitive::Int => {
                    let v = self.object.extract().map_err(ser)?;
                    s.serialize_i64(v)
                }
                Primitive::Str => {
                    let v = self.object.extract().map_err(ser)?;
                    s.serialize_str(v)
                }
                Primitive::Float => {
                    let v = self.object.extract().map_err(ser)?;
                    s.serialize_f64(v)
                }
                Primitive::ByteArray => {
                    let v: Vec<u8> = self.object.extract().map_err(ser)?;
                    s.serialize_bytes(&v)
                }
                Primitive::Bytes => {
                    let v = self.object.extract().map_err(ser)?;
                    s.serialize_bytes(v)
                }
            },
            Schema::List(l) => {
                let list: &PyList = self.object.extract().map_err(ser)?;
                let mut seq = s.serialize_seq(Some(list.len()))?;
                for v in list {
                    let object = TypedObject::new(&l.value, v);
                    seq.serialize_element(&object)?;
                }
                seq.end()
            }
            Schema::Set(l) => {
                let set: &PySet = self.object.extract().map_err(ser)?;
                let mut seq = s.serialize_seq(Some(set.len()))?;
                for v in set {
                    let object = TypedObject::new(&l.value, v);
                    seq.serialize_element(&object)?;
                }
                seq.end()
            }
            Schema::Tuple(t) => {
                let tuple: &PyTuple = self.object.extract().map_err(ser)?;
                let mut seq = s.serialize_seq(Some(tuple.len()))?;

                if t.args.len() != tuple.len() {
                    return Err(S::Error::custom(format!(
                        "tuple expects {} args but got {}",
                        t.args.len(),
                        tuple.len()
                    )));
                }

                for (obj, schema) in tuple.into_iter().zip(t.args.iter()) {
                    let object = TypedObject::new(schema, obj);
                    seq.serialize_element(&object)?;
                }
                seq.end()
            }
            Schema::Dict(d) => {
                let dict: &PyDict = self.object.extract().map_err(ser)?;
                let mut map = s.serialize_map(Some(dict.len()))?;
                for (k, v) in dict {
                    let k = TypedObject::new(&*d.key, k);
                    let v = TypedObject::new(&*d.value, v);
                    map.serialize_entry(&k, &v)?;
                }
                map.end()
            }
            Schema::Class(c) => {
                let v: &PyAny = self.object.extract().map_err(ser)?;
                let mems = c.retrieve_members(v)?;
                let mut map = s.serialize_map(Some(mems.len()))?;
                for (k, t) in mems {
                    map.serialize_entry(&k, &t)?;
                }
                map.end()
            }
            Schema::Enum(e) => {
                let v: &PyAny = self.object.extract().map_err(ser)?;
                let name = e.verify_variant(v)?;
                name.serialize(s)
            }
            Schema::Optional(o) => {
                let some: Option<&PyAny> = self.object.extract().map_err(ser)?;
                match some {
                    Some(v) => {
                        let v = TypedObject::new(&o.value, v);
                        s.serialize_some(&v)
                    }
                    None => s.serialize_none(),
                }
            }
            Schema::Union(u) => {
                let v: &PyAny = self.object.extract().map_err(ser)?;
                let schema = u.find_union_variant(v)?;
                let v = TypedObject::new(schema, v);
                v.serialize(s)
            }
            Schema::Any(a) => unimplemented!(),
        }
    }
}

impl Class {
    #[cfg_attr(feature = "perf", flame)]
    pub fn retrieve_members<'a, E>(
        &'a self,
        value: &'a PyAny,
    ) -> Result<Vec<(&'a str, TypedObject<'a>)>, E>
    where
        E: ser::Error,
    {
        self.fields.iter().try_fold(vec![], |mut mems, (k, f)| {
            if f.attr.flatten {
                match &f.schema {
                    Schema::Class(c) => {
                        let v = value.getattr(&f.name).map_err(ser)?;
                        mems.extend(c.retrieve_members(v)?);
                    }
                    _ => {
                        return Err(ser::Error::custom(format!(
                            "found flatten attribute to non-map type"
                        )))
                    }
                }
            } else {
                let v = value.getattr(&f.name).map_err(ser)?;
                mems.push((k.as_ref(), TypedObject::new(&f.schema, v)))
            }
            Ok(mems)
        })
    }
}

impl Enum {
    #[cfg_attr(feature = "perf", flame)]
    pub fn verify_variant<'a, E>(&self, value: &'a PyAny) -> Result<&'a str, E>
    where
        E: ser::Error,
    {
        let name: &str = value
            .getattr("name")
            .and_then(|v| v.extract())
            .map_err(ser)?;
        self.variants
            .get(name)
            .ok_or_else(|| ser::Error::custom(format!("unknown variant `{}`", name)))?;
        Ok(name)
    }
}

impl Union {
    #[cfg_attr(feature = "perf", flame)]
    pub fn find_union_variant<'a, E>(&'a self, value: &'a PyAny) -> Result<&'a Schema, E>
    where
        E: ser::Error,
    {
        self.variants
            .iter()
            .find(|s| s.type_of(value).unwrap_or(false))
            .ok_or_else(|| {
                ser::Error::custom(format!("unknown variant `{}`", value.get_type().name()))
            })
    }
}
