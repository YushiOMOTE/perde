use perde_core::prelude::*;

fn loads_as_(schema: &Schema, object: &ObjectRef) -> Result<Object> {
    let s = object.as_str()?;
    Ok(serde_yaml::seed::from_str_seed(s, schema)?)
}

fn loads_(object: &ObjectRef) -> Result<Object> {
    let s = object.as_str()?;
    Ok(serde_yaml::from_str(s)?)
}

fn dumps_(object: WithSchema) -> Result<Object> {
    let s = serde_yaml::to_string(&object)?;
    Ok(Object::new_str(&s)?)
}

impl_default_methods! {
    perde_yaml,
    YamlError,
    loads_as_,
    loads_,
    dumps_
}
