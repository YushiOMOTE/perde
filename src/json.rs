use crate::{
    schema::Schema,
    types::{self, Object, Tuple},
    util::*,
};
use pyo3::{ffi::*, prelude::*, wrap_pyfunction, wrap_pymodule};
use serde::Deserialize;
use std::os::raw::c_char;

pub unsafe extern "C" fn loads_as(
    _self: *mut pyo3::ffi::PyObject,
    args: *mut pyo3::ffi::PyObject,
) -> *mut pyo3::ffi::PyObject {
    let args = objnew!(args).unwrap();
    let args = Tuple::from(args);

    let mut size = 0;
    let p = crate::unicode::read_utf8_from_str(args.getref(1).unwrap().as_ptr(), &mut size);
    let s = unsafe { std::slice::from_raw_parts(p, size as usize) };
    let mut deserializer = serde_json::Deserializer::from_slice(s);

    let obj = Schema::deserialize(args.getref(0).unwrap(), &mut deserializer).unwrap();

    #[cfg(feature = "perf")]
    {
        flame::dump_html(&mut std::fs::File::create("flame-graph.html").unwrap()).unwrap();
        flame::clear();
    }

    obj.into_ptr()
}

// #[pyfunction]
// pub fn dumps(v: &PyAny) -> PyResult<String> {
//     let buf = vec![];
//     let mut serializer = serde_json::Serializer::new(buf);
//     Schema::serialize(v, &mut serializer)?;
//     let buf = serializer.into_inner();

//     #[cfg(feature = "perf")]
//     {
//         flame::dump_html(&mut std::fs::File::create("flame-graph.html").unwrap()).unwrap();
//         flame::clear();
//     }

//     Ok(String::from_utf8(buf)?)
// }

// pub unsafe extern "C" fn loads(
//     _self: *mut pyo3::ffi::PyObject,
//     obj: *mut pyo3::ffi::PyObject,
// ) -> *mut pyo3::ffi::PyObject {
//     use pyo3::AsPyPointer;
//     let mut size = 0;
//     let p = crate::unicode::read_utf8_from_str(obj, &mut size);
//     let s = unsafe { std::slice::from_raw_parts(p, size as usize) };
//     let mut deserializer = serde_json::Deserializer::from_slice(s);
//     Object::deserialize(&mut deserializer)
//         .map_err(pyerr)
//         .map(|v| v.into_ptr())
//         .unwrap_or_else(|_| std::ptr::null_mut())
// }

#[pymodule]
pub fn json(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    use pyo3::AsPyPointer;

    // m.add_wrapped(wrap_pyfunction!(dumps))?;

    // let def = pyo3::ffi::PyMethodDef {
    //     ml_name: "loads\0".as_ptr() as *const c_char,
    //     ml_meth: Some(loads),
    //     ml_flags: pyo3::ffi::METH_O,
    //     ml_doc: "".as_ptr() as *const c_char,
    // };
    // unsafe {
    //     pyo3::ffi::PyModule_AddObject(
    //         m.as_ptr(),
    //         "loads\0".as_ptr() as *const c_char,
    //         pyo3::ffi::PyCFunction_New(Box::into_raw(Box::new(def)), std::ptr::null_mut()),
    //     )
    // };

    let def = pyo3::ffi::PyMethodDef {
        ml_name: "loads_as\0".as_ptr() as *const c_char,
        ml_meth: Some(loads_as),
        ml_flags: pyo3::ffi::METH_VARARGS,
        ml_doc: "".as_ptr() as *const c_char,
    };
    unsafe {
        pyo3::ffi::PyModule_AddObject(
            m.as_ptr(),
            "loads_as\0".as_ptr() as *const c_char,
            pyo3::ffi::PyCFunction_New(Box::into_raw(Box::new(def)), std::ptr::null_mut()),
        )
    };

    Ok(())
}

pub fn import(m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(json))
}
