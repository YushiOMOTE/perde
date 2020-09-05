use crate::{
    ser::TypedObject,
    state::DeserializeState,
    types::{Object, Schema},
    util::*,
};
use pyo3::{prelude::*, types::PyDict, wrap_pyfunction};
use serde::{Deserialize, Serialize};

#[cfg(feature = "perf")]
#[macro_use]
extern crate flamer;

mod de;
mod ser;
mod state;
mod types;
mod util;

#[cfg(feature = "json")]
#[pyfunction]
pub fn loads_as(ty: &PyAny, s: &str) -> PyResult<PyObject> {
    let schema = Schema::resolve(ty)?;
    let schema = schema.borrow_mut();
    let mut deserializer = serde_json::Deserializer::from_str(s);
    let obj: Object = Object::deserialize_state(&*schema, &mut deserializer).map_err(pyerr)?;

    #[cfg(feature = "perf")]
    {
        flame::dump_html(&mut std::fs::File::create("flame-graph.html").unwrap()).unwrap();
        flame::clear();
    }

    Ok(obj.into())
}

#[cfg(feature = "json")]
#[pyfunction]
pub fn dumps(v: &PyAny) -> PyResult<String> {
    let schema = Schema::resolve(v.get_type())?;
    let schema = schema.borrow();
    let object = v.to_object(py());
    let object = TypedObject::new(&*schema, object);
    // let schema = schema.borrow_mut();
    let buf = vec![];
    let mut serializer = serde_json::Serializer::new(buf);
    object.serialize(&mut serializer).map_err(pyerr)?;
    let buf = serializer.into_inner();

    #[cfg(feature = "perf")]
    {
        flame::dump_html(&mut std::fs::File::create("flame-graph.html").unwrap()).unwrap();
        flame::clear();
    }

    Ok(String::from_utf8(buf)?)
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
    #[cfg(feature = "json")]
    m.add_wrapped(wrap_pyfunction!(dumps))?;

    Ok(())
}
