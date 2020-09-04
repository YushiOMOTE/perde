use crate::{
    types::{Object, Schema, SchemaStack},
    util::*,
};
use pyo3::{prelude::*, wrap_pyfunction};
use serde::Deserialize;
use serde_state::de::DeserializeState;

mod de;
mod types;
mod util;

#[cfg(feature = "json")]
#[pyfunction]
pub fn load_as(ty: &PyAny, s: &str) -> PyResult<PyObject> {
    let schema = Schema::resolve(ty)?;
    let schema = schema.borrow();
    let mut stack = SchemaStack::new(&schema);
    let mut deserializer = serde_json::Deserializer::from_str(s);
    let obj: Object = Object::deserialize_state(&mut stack, &mut deserializer).map_err(pyerr)?;
    Ok(obj.into())
}

#[cfg(feature = "json")]
#[pyfunction]
pub fn loads(s: &str) -> PyResult<PyObject> {
    let mut deserializer = serde_json::Deserializer::from_str(s);
    let obj = Object::deserialize(&mut deserializer).map_err(pyerr)?;
    Ok(obj.into())
}

// load!("json", load_as, serde_json);
// load!("yaml", yaml_load, serde_yaml);
// load!("toml", toml_load, serde_toml);

// #[cfg(feature = "msgpack")]
// #[pyfunction]
// pub fn msgpack_load(s: &[u8]) -> PyResult<PyObject> {
//     let obj: Object = rmp_serde::from_slice(s).map_err(pyerr)?;
//     Ok(obj.into())
// }

#[pymodule]
fn perde(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Schema>()?;

    #[cfg(feature = "json")]
    m.add_wrapped(wrap_pyfunction!(load_as))?;
    #[cfg(feature = "json")]
    m.add_wrapped(wrap_pyfunction!(loads))?;
    // #[cfg(feature = "yaml")]
    // m.add_wrapped(wrap_pyfunction!(yaml_load))?;
    // #[cfg(feature = "toml")]
    // m.add_wrapped(wrap_pyfunction!(toml_load))?;
    // #[cfg(feature = "msgpack")]
    // m.add_wrapped(wrap_pyfunction!(msgpack_load))?;

    Ok(())
}
