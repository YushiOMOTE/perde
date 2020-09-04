use crate::{
    state::DeserializeState,
    types::{Object, Schema},
    util::*,
};
use pyo3::{prelude::*, types::PyDict, wrap_pyfunction};
use serde::Deserialize;

#[macro_use]
extern crate flamer;

mod de;
mod state;
mod types;
mod util;

#[cfg(feature = "json")]
#[pyfunction]
pub fn loads_as(ty: &PyAny, s: &str) -> PyResult<PyObject> {
    // flame::start("load_as");
    let schema = Schema::resolve(ty)?;
    let schema = schema.borrow_mut();
    let mut deserializer = serde_json::Deserializer::from_str(s);
    let obj: Object = Object::deserialize_state(&*schema, &mut deserializer).map_err(pyerr)?;
    // flame::end("load_as");

    // flame::dump_html(&mut std::fs::File::create("flame-graph.html").unwrap()).unwrap();
    // flame::clear();
    Ok(obj.into())
}

#[cfg(feature = "json")]
#[pyfunction(kwargs = "**")]
pub fn loads(s: &str, kwargs: Option<&PyDict>) -> PyResult<PyObject> {
    if let Some(kwargs) = kwargs {
        if let Some(ty) = kwargs.get_item("type") {
            return loads_as(ty, s);
        }
    }

    let mut deserializer = serde_json::Deserializer::from_str(s);
    let obj = Object::deserialize(&mut deserializer).map_err(pyerr)?;
    Ok(obj.into())
}

#[pyfunction]
pub fn f() -> PyResult<()> {
    Ok(())
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
    m.add_wrapped(wrap_pyfunction!(loads_as))?;
    #[cfg(feature = "json")]
    m.add_wrapped(wrap_pyfunction!(loads))?;

    m.add_wrapped(wrap_pyfunction!(f))?;
    // #[cfg(feature = "yaml")]
    // m.add_wrapped(wrap_pyfunction!(yaml_load))?;
    // #[cfg(feature = "toml")]
    // m.add_wrapped(wrap_pyfunction!(toml_load))?;
    // #[cfg(feature = "msgpack")]
    // m.add_wrapped(wrap_pyfunction!(msgpack_load))?;

    Ok(())
}
