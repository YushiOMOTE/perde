use perde_core::prelude::*;

fn loads_as(schema: &Schema, object: &ObjectRef) -> Result<Object> {
    let s = object.as_str().context("invalid argument")?;
    Ok(serde_yaml::seed::from_str_seed(s, schema)?)
}

fn loads(object: &ObjectRef) -> Result<Object> {
    let s = object.as_str().context("invalid argument")?;
    Ok(serde_yaml::from_str(s)?)
}

fn dumps(object: WithSchema) -> Result<Object> {
    let s = serde_yaml::to_string(&object)?;
    Ok(Object::new_str(&s)?)
}

impl_default_methods! {
    yaml,
    YamlError,
    loads_as,
    loads,
    dumps
}
