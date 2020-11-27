use perde_core::prelude::*;
use std::borrow::Cow;

fn loads_as_<'a>(schema: Cow<'a, Schema>, object: &ObjectRef) -> Result<Object> {
    let s = object.as_str().context("invalid argument")?;
    Ok(serde_yaml::seed::from_str_seed(s, schema.as_ref())?)
}

fn loads_(object: &ObjectRef) -> Result<Object> {
    let s = object.as_str().context("invalid argument")?;
    Ok(serde_yaml::from_str(s)?)
}

fn dumps_(object: WithSchema) -> Result<Object> {
    let s = serde_yaml::to_string(&object)?;
    Ok(Object::new_str(&s)?)
}

impl_default_methods! {
    yaml,
    YamlError,
    loads_as_,
    loads_,
    dumps_
}
