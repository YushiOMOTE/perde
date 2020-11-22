use perde_core::prelude::*;
use serde::de::DeserializeSeed;

fn loads_as_(schema: &Schema, object: &ObjectRef) -> Result<Object> {
    let b = object.as_bytes().context("invalid argument")?;
    let mut de = rmp_serde::Deserializer::from_read_ref(&b);
    Ok(schema.deserialize(&mut de)?)
}

fn loads_(object: &ObjectRef) -> Result<Object> {
    let b = object.as_bytes().context("invalid argument")?;
    Ok(rmp_serde::from_slice(&b)?)
}

fn dumps_(object: WithSchema) -> Result<Object> {
    Object::new_bytes(&rmp_serde::to_vec(&object)?)
}

impl_default_methods! {
    msgpack,
    MsgpackError,
    loads_as_,
    loads_,
    dumps_
}
