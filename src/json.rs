use crate::{
    ser::TypedObject,
    state::DeserializeState,
    types::{Object, Schema},
    util::*,
};
use pyo3::{prelude::*, wrap_pyfunction, wrap_pymodule};
use serde::{Deserialize, Serialize};

#[pyfunction]
pub fn loads_as(ty: &PyAny, s: &str) -> PyResult<PyObject> {
    let obj: Object = Schema::with(ty, |schema| {
        let mut deserializer = serde_json::Deserializer::from_str(s);
        Object::deserialize_state(schema, &mut deserializer).map_err(pyerr)
    })?;

    #[cfg(feature = "perf")]
    {
        flame::dump_html(&mut std::fs::File::create("flame-graph.html").unwrap()).unwrap();
        flame::clear();
    }

    Ok(obj.into())
}

#[pyfunction]
pub fn dumps(v: &PyAny) -> PyResult<String> {
    let ty = v.get_type();
    let buf = Schema::with(ty, |schema| {
        let object = v.to_object(py());
        let object = TypedObject::new(&*schema, object);

        let buf = vec![];
        let mut serializer = serde_json::Serializer::new(buf);
        object.serialize(&mut serializer).map_err(pyerr)?;
        Ok(serializer.into_inner())
    })?;

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
    let obj = Object::deserialize(&mut deserializer).map_err(pyerr)?;
    Ok(obj.into())
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
