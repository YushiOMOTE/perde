use crate::{object::Object, schema::Schema, util::*};
use pyo3::{prelude::*, wrap_pyfunction, wrap_pymodule};
use serde::Deserialize;
use std::os::raw::c_char;

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

pub unsafe extern "C" fn loads(
    _self: *mut pyo3::ffi::PyObject,
    obj: *mut pyo3::ffi::PyObject,
) -> *mut pyo3::ffi::PyObject {
    use pyo3::AsPyPointer;
    let mut size = 0;
    let p = crate::unicode::read_utf8_from_str(obj, &mut size);
    let s = unsafe { std::slice::from_raw_parts(p, size as usize) };
    let mut deserializer = serde_json::Deserializer::from_slice(s);
    Object::deserialize(&mut deserializer)
        .map_err(pyerr)
        .map(|v| v.to_pyobj().as_ptr())
        .unwrap_or_else(|_| std::ptr::null_mut())
}

#[pymodule]
pub fn json(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    use pyo3::AsPyPointer;

    m.add_wrapped(wrap_pyfunction!(loads_as))?;
    m.add_wrapped(wrap_pyfunction!(dumps))?;

    let def = pyo3::ffi::PyMethodDef {
        ml_name: "loads\0".as_ptr() as *const c_char,
        ml_meth: Some(loads),
        ml_flags: pyo3::ffi::METH_O,
        ml_doc: "".as_ptr() as *const c_char,
    };
    unsafe {
        pyo3::ffi::PyModule_AddObject(
            m.as_ptr(),
            "loads\0".as_ptr() as *const c_char,
            pyo3::ffi::PyCFunction_New(Box::into_raw(Box::new(def)), std::ptr::null_mut()),
        )
    };

    Ok(())
}

pub fn import(m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(json))
}
