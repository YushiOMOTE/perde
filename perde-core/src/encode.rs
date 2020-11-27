use crate::{
    attr::AttrStr,
    error::Convert,
    object::ObjectRef,
    schema::{Any, FieldSchema, Schema, WithSchema},
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
            Schema::Bool => s.serialize_bool(self.object.as_bool().ser()?),
            Schema::Int => s.serialize_i64(self.object.as_i64().ser()?),
            Schema::Str => s.serialize_str(self.object.as_str().ser()?),
            Schema::Float => s.serialize_f64(self.object.as_f64().ser()?),
            Schema::ByteArray => s.serialize_bytes(self.object.as_bytearray().ser()?),
            Schema::Bytes => s.serialize_bytes(self.object.as_bytes().ser()?),
            Schema::DateTime => s.serialize_str(self.object.isoformat().ser()?.as_str().ser()?),
            Schema::Time => s.serialize_str(self.object.isoformat().ser()?.as_str().ser()?),
            Schema::Date => s.serialize_str(self.object.isoformat().ser()?.as_str().ser()?),
            Schema::Decimal => s.serialize_str(self.object.to_str().ser()?.as_str().ser()?),
            Schema::Uuid => s.serialize_str(self.object.to_str().ser()?.as_str().ser()?),
            Schema::List(l) => {
                let len = self.object.as_list().len();
                let mut seq = s.serialize_seq(Some(len))?;

                for item in self.object.get_iter().ser()? {
                    let item = item.ser()?;
                    let w = item.with_schema(&l.value);
                    seq.serialize_element(&w)?;
                }
                seq.end()
            }
            Schema::Set(l) => {
                let len = self.object.as_set().len();
                let mut seq = s.serialize_seq(Some(len))?;

                for item in self.object.get_iter().ser()? {
                    let item = item.ser()?;
                    let w = item.with_schema(&l.value);
                    seq.serialize_element(&w)?;
                }
                seq.end()
            }
            Schema::FrozenSet(l) => {
                let len = self.object.as_set().len();
                let mut seq = s.serialize_seq(Some(len))?;

                for item in self.object.get_iter().ser()? {
                    let item = item.ser()?;
                    let w = item.with_schema(&l.value);
                    seq.serialize_element(&w)?;
                }
                seq.end()
            }
            Schema::Tuple(t) => {
                let iter = self.object.get_tuple_iter().ser()?;
                let mut seq = s.serialize_seq(Some(iter.len()))?;
                if t.any {
                    for obj in iter {
                        let w = obj.with_schema(&Schema::Any(Any));
                        seq.serialize_element(&w)?;
                    }
                } else {
                    for (obj, schema) in iter.zip(t.args.iter()) {
                        let w = obj.with_schema(schema);
                        seq.serialize_element(&w)?;
                    }
                }
                seq.end()
            }
            Schema::Dict(d) => {
                let dict = self.object.get_dict_iter().ser()?;
                let mut map = s.serialize_map(Some(dict.len()))?;
                for (k, v) in dict {
                    let k = k.with_schema(&d.key);
                    let v = v.with_schema(&d.value);
                    map.serialize_entry(&k, &v)?;
                }
                map.end()
            }
            Schema::Class(c) => {
                let len = if c.flatten_dict.is_some() {
                    None
                } else {
                    Some(c.ser_field_len)
                };
                let mut map = s.serialize_map(len)?;
                serialize_fields(&self.object, &c.fields, &mut map)?;
                map.end()
            }
            Schema::Enum(e) => {
                if e.attr.as_value {
                    let value = self.object.get_attr(&ATTR_VALUE).ser()?;
                    value.resolved_object().ser()?.serialize(s)
                } else {
                    let name = self.object.get_attr(&ATTR_NAME).ser()?;
                    let name = name.as_str().ser()?;
                    let e = match e.variants.iter().find(|s| s.name == name) {
                        Some(e) => e,
                        None => return Err(S::Error::custom(format!("no such variant: {}", name))),
                    };
                    if e.attr.skip || e.attr.skip_serializing {
                        return Err(S::Error::custom(format!(
                            "variant `{}` is marked as `skip` and cannot be serialized",
                            name
                        )));
                    }
                    s.serialize_str(&e.sername)
                }
            }
            Schema::Union(u) => {
                if self.object.is_none() && u.optional {
                    return s.serialize_none();
                }
                let vs = self.object.get_type().ser()?.resolve(None).ser()?;
                let vs = match u.variants.iter().find(|v| v == &vs) {
                    Some(vs) => vs,
                    None => return Err(S::Error::custom("no such variant".to_string())),
                };
                let v = self.object.with_schema(vs);
                v.serialize(s)
            }
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
    for (_, field) in fields {
        if field.attr.skip || field.attr.skip_serializing {
            continue;
        }

        let obj = object.get_attr(&field.name).ser()?;

        if field.attr.flatten {
            match &field.schema {
                Schema::Class(cls) => {
                    serialize_fields(&obj, &cls.fields, map)?;
                }
                Schema::Dict(d) => {
                    let dict = obj.get_dict_iter().ser()?;
                    for (k, v) in dict {
                        let k = k.with_schema(&d.key);
                        let v = v.with_schema(&d.value);
                        map.serialize_entry(&k, &v)?;
                    }
                }
                _ => return Err(E::custom("found flatten flag for non-class type".to_string())),
            }
        } else {
            let f = obj.with_schema(&field.schema);
            map.serialize_entry(&field.rename, &f)?;
        }
    }
    Ok(())
}
