use crate::{
    error::Convert,
    schema::{FieldSchema, Primitive, Schema, WithSchema},
    types::{AttrStr, DictRef, ListRef, ObjectRef, Set, SetRef, TupleRef},
};
use indexmap::IndexMap;
use serde::ser::Error;
use serde::{
    ser::{SerializeMap, SerializeSeq, Serializer},
    Serialize,
};

lazy_static::lazy_static! {
    static ref ATTR_NAME: AttrStr = AttrStr::new("name");
    static ref ATTR_VALUE: AttrStr = AttrStr::new("value");
}

impl<'a> Serialize for WithSchema<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.schema {
            Schema::Primitive(Primitive::Bool) => s.serialize_bool(self.object.as_bool().ser()?),
            Schema::Primitive(Primitive::Int) => s.serialize_i64(self.object.as_i64().ser()?),
            Schema::Primitive(Primitive::Str) => s.serialize_str(self.object.as_str().ser()?),
            Schema::Primitive(Primitive::Float) => s.serialize_f64(self.object.as_f64().ser()?),
            Schema::Primitive(Primitive::ByteArray) => {
                s.serialize_bytes(self.object.as_bytearray().ser()?)
            }
            Schema::Primitive(Primitive::Bytes) => s.serialize_bytes(self.object.as_bytes().ser()?),
            Schema::List(l) => {
                let list = ListRef::new(self.object);
                let len = list.len();
                let mut seq = s.serialize_seq(Some(len))?;
                for i in 0..len {
                    let obj = list.get(i).unwrap();
                    let w = obj.with_schema(&l.value);
                    seq.serialize_element(&w)?;
                }
                seq.end()
            }
            Schema::Set(l) => {
                let set = SetRef::new(self.object);
                let len = set.len();
                let mut seq = s.serialize_seq(Some(len))?;

                while let Some(item) = set.pop() {
                    let w = item.with_schema(&l.value);
                    seq.serialize_element(&w)?;
                }
                seq.end()
            }
            Schema::FrozenSet(l) => {
                let set = Set::from_iter(self.object).ser()?;
                let len = set.len();
                let mut seq = s.serialize_seq(Some(len))?;

                while let Some(item) = set.pop() {
                    let w = item.with_schema(&l.value);
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
                    let w = obj.with_schema(schema);
                    seq.serialize_element(&w)?;
                }
                seq.end()
            }
            Schema::Dict(d) => {
                let dict = DictRef::new(self.object);
                let mut map = s.serialize_map(Some(dict.len()))?;
                for (k, v) in dict.iter() {
                    let k = k.with_schema(&d.key);
                    let v = v.with_schema(&d.value);
                    map.serialize_entry(&k, &v)?;
                }
                map.end()
            }
            Schema::Class(c) => {
                let mut map = s.serialize_map(Some(c.fields.len()))?;
                serialize_fields(&self.object, &c.fields, &mut map)?;
                map.end()
            }
            Schema::Enum(e) => {
                let name = self.object.get_attr(&ATTR_NAME).ser()?;
                let name = name.as_str().ser()?;
                if !e.variants.contains_key(name) {
                    return Err(S::Error::custom(format!("no such variant: {}", name)));
                }
                s.serialize_str(&name)
            }
            Schema::Optional(o) => {
                if self.object.is_none() {
                    s.serialize_none()
                } else {
                    let w = self.object.with_schema(&o.value);
                    s.serialize_some(&w)
                }
            }
            Schema::Union(_u) => unimplemented!(),
            Schema::Any(_) => self.object.resolved_object().ser()?.serialize(s),
        }
    }
}

fn serialize_fields<T, E>(
    object: &ObjectRef,
    fields: &IndexMap<String, FieldSchema>,
    map: &mut T,
) -> Result<(), E>
where
    T: SerializeMap<Error = E>,
    E: serde::ser::Error,
{
    for (name, field) in fields {
        let obj = object.get_attr(&field.name).ser()?;
        if field.attr.flatten {
            match &field.schema {
                Schema::Class(cls) => {
                    serialize_fields(&obj, &cls.fields, map)?;
                }
                _ => return Err(E::custom(format!("found flatten flag for non-class type"))),
            }
        } else {
            let f = obj.with_schema(&field.schema);
            map.serialize_entry(&name, &f)?;
        }
    }
    Ok(())
}
