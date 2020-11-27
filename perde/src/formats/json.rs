use perde_core::prelude::*;
use serde::de::DeserializeSeed;
use std::borrow::Cow;

fn loads_as_<'a>(schema: Cow<'a, Schema>, object: &ObjectRef) -> Result<Object> {
    let s = object.as_str().context("invalid argument")?;
    let mut de = serde_json::Deserializer::from_str(&s);
    Ok(schema.as_ref().deserialize(&mut de)?)
}

fn loads_(object: &ObjectRef) -> Result<Object> {
    let s = object.as_str().context("invalid argument")?;
    Ok(serde_json::from_str(s)?)
}

fn dumps_(object: WithSchema) -> Result<Object> {
    Object::new_str(&serde_json::to_string(&object)?)
}

impl_default_methods! {
    json,
    JsonError,
    loads_as_,
    loads_,
    dumps_
}
