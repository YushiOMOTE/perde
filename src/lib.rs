use crate::{
    state::DeserializeState,
    types::{Object, Schema},
    util::*,
};
use pyo3::{prelude::*, types::PyDict, wrap_pyfunction};
use serde::Deserialize;

#[cfg(feature = "perf")]
#[macro_use]
extern crate flamer;

mod de;
mod state;
mod types;
mod util;

#[cfg_attr(feature = "perf", flame)]
fn test1_visit_u64() {}

#[cfg_attr(feature = "perf", flame)]
fn test_visit_u64() {
    test1_visit_u64()
}

#[cfg(feature = "json")]
#[pyfunction]
pub fn loads_as(ty: &PyAny, s: &str) -> PyResult<PyObject> {
    let schema = Schema::resolve(ty)?;
    let schema = schema.borrow_mut();
    let mut deserializer = serde_json::Deserializer::from_str(s);
    let obj: Object = Object::deserialize_state(&*schema, &mut deserializer).map_err(pyerr)?;

    test_visit_u64();

    #[cfg(feature = "perf")]
    {
        flame::dump_html(&mut std::fs::File::create("flame-graph.html").unwrap()).unwrap();
        flame::clear();
    }

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

#[pymodule]
fn perde(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Schema>()?;

    #[cfg(feature = "json")]
    m.add_wrapped(wrap_pyfunction!(loads_as))?;
    #[cfg(feature = "json")]
    m.add_wrapped(wrap_pyfunction!(loads))?;

    Ok(())
}
