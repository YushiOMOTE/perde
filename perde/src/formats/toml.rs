use perde_core::prelude::*;
use serde::de::DeserializeSeed;

fn loads_as(schema: &Schema, object: &ObjectRef) -> Result<Object> {
    let buf = object.as_str().context("invalid argument")?;
    let mut deserializer = toml::de::Deserializer::new(&buf);
    Ok(schema.deserialize(&mut deserializer)?)
}

fn loads(object: &ObjectRef) -> Result<Object> {
    let s = object.as_str().context("invalid argument")?;
    Ok(toml::from_str(&s)?)
}

fn dumps(object: WithSchema) -> Result<Object> {
    Object::new_str(&toml::to_string(&object)?)
}

impl_default_methods! {
    toml,
    TomlError,
    loads_as,
    loads,
    dumps
}
