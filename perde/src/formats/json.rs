use perde_core::prelude::*;
use serde::de::DeserializeSeed;

fn loads_as(schema: &Schema, object: &ObjectRef) -> Result<Object> {
    let s = object.as_str().context("invalid argument")?;
    let mut de = serde_json::Deserializer::from_str(&s);
    Ok(schema.deserialize(&mut de)?)
}

fn loads(object: &ObjectRef) -> Result<Object> {
    let s = object.as_str().context("invalid argument")?;
    Ok(serde_json::from_str(s)?)
}

fn dumps(object: WithSchema) -> Result<Object> {
    Object::new_str(&serde_json::to_string(&object)?)
}

impl_default_methods! {
    json,
    JsonError,
    loads_as,
    loads,
    dumps
}
