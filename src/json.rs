use crate::{object::Object, schema::Schema, ser::TypedObject, state::DeserializeState, util::*};
use pyo3::{prelude::*, wrap_pyfunction, wrap_pymodule};
use serde::{Deserialize, Serialize};

#[pyfunction]
pub fn loads_as(ty: &PyAny, s: &str) -> PyResult<PyObject> {
    let mut deserializer = serde_json::Deserializer::from_str(s);
    let obj = Schema::deserialize(ty, &mut deserializer)?;

    #[cfg(feature = "perf")]
    {
        flame::dump_html(&mut std::fs::File::create("flame-graph.html").unwrap()).unwrap();
        flame::clear();
    }

    Ok(obj)
}

#[pyfunction]
pub fn dumps(v: &PyAny) -> PyResult<String> {
    let buf = vec![];
    let mut serializer = serde_json::Serializer::new(buf);
    Schema::serialize(v, &mut serializer)?;
    let buf = serializer.into_inner();

    #[cfg(feature = "perf")]
    {
        flame::dump_html(&mut std::fs::File::create("flame-graph.html").unwrap()).unwrap();
        flame::clear();
    }

    Ok(String::from_utf8(buf)?)
}

#[pyfunction(kwargs = "**")]
pub fn loads(s: &str) -> PyResult<PyObject> {
    let mut deserializer = serde_json::Deserializer::from_str(s);
    Object::deserialize(&mut deserializer)
        .map_err(pyerr)
        .map(|v| v.to_pyobj())
}

#[pymodule]
pub fn json(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(loads_as))?;
    m.add_wrapped(wrap_pyfunction!(loads))?;
    m.add_wrapped(wrap_pyfunction!(dumps))?;

    Ok(())
}

pub fn import(m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(json))
}
