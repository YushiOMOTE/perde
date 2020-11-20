use perde_core::prelude::*;
use serde::de::DeserializeSeed;

fn loads_as_(schema: &Schema, object: &ObjectRef) -> Result<Object> {
    let s = object.as_str().context("invalid argument")?;
    let mut de = serde_json::Deserializer::from_str(&s);
    Ok(schema.deserialize(&mut de)?)
}

fn loads_(object: &ObjectRef) -> Result<Object> {
    let s = object.as_str().context("invalid argument")?;
    Ok(serde_json::from_str(s)?)
}

fn dumps_(object: WithSchema) -> Result<Object> {
    Object::new_str(&serde_json::to_string(&object)?)
}

impl_default_methods! {
    perde_json,
    JsonError,
    loads_as_,
    loads_,
    dumps_
}
