use crate::types::_PyCFunctionFastWithKeywords;
use pyo3::{conversion::AsPyPointer, ffi::*, prelude::*};
use std::os::raw::c_char;

#[cfg(feature = "perf")]
#[macro_use]
extern crate flamer;

#[macro_use]
mod error;

#[macro_use]
mod types;
// mod util;

mod inspect;
mod schema;

mod decode;
mod encode;

#[cfg(feature = "json")]
mod json;

mod unicode;

// pub unsafe extern "C" fn resolve(
//     _self: *mut pyo3::ffi::PyObject,
//     args: *const *mut pyo3::ffi::PyObject,
//     nargs: Py_ssize_t,
//     kwnames: *mut pyo3::ffi::PyObject,
// ) -> *mut pyo3::ffi::PyObject {
//     unimplemented!()
// }

#[pymodule]
fn perde(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[cfg(feature = "json")]
    json::import(m)?;

    // let def = pyo3::ffi::PyMethodDef {
    //     ml_name: "resolve\0".as_ptr() as *const c_char,
    //     ml_meth: Some(unsafe {
    //         std::mem::transmute::<_PyCFunctionFastWithKeywords, PyCFunction>(resolve)
    //     }),
    //     ml_flags: pyo3::ffi::METH_FASTCALL | pyo3::ffi::METH_KEYWORDS,
    //     ml_doc: "".as_ptr() as *const c_char,
    // };
    // unsafe {
    //     pyo3::ffi::PyModule_AddObject(
    //         m.as_ptr(),
    //         "resolve\0".as_ptr() as *const c_char,
    //         pyo3::ffi::PyCFunction_New(Box::into_raw(Box::new(def)), std::ptr::null_mut()),
    //     )
    // };

    Ok(())
}
