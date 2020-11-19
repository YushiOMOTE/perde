use perde_core::prelude::*;
use serde::de::DeserializeSeed;

fn loads_as_(schema: &Schema, object: &ObjectRef) -> Result<Object> {
    let buf = object.as_str()?;
    let mut deserializer = toml::de::Deserializer::new(&buf);
    Ok(schema.deserialize(&mut deserializer)?)
}

fn loads_(object: &ObjectRef) -> Result<Object> {
    let s = object.as_str()?;
    Ok(toml::from_str(&s)?)
}

fn dumps_(object: WithSchema) -> Result<Object> {
    Object::new_str(&toml::to_string(&object)?)
}

impl_default_methods! {
    perde_toml,
    TomlError,
    loads_as_,
    loads_,
    dumps_
}
