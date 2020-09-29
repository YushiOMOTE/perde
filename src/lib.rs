use crate::{
    error::Convert,
    types::{Object, ObjectRef, TupleRef, _PyCFunctionFastWithKeywords},
};
use pyo3::{conversion::AsPyPointer, ffi::*, prelude::*};
use std::{collections::HashMap, os::raw::c_char};

#[macro_use]
mod error;

#[macro_use]
mod types;

mod inspect;
mod schema;

mod decode;
mod encode;

#[cfg(feature = "json")]
mod json;

pub unsafe extern "C" fn resolve(
    _self: *mut pyo3::ffi::PyObject,
    args: *const *mut pyo3::ffi::PyObject,
    nargs: Py_ssize_t,
    kwnames: *mut pyo3::ffi::PyObject,
) -> *mut pyo3::ffi::PyObject {
    let inner = || {
        let nargs = PyVectorcall_NARGS(nargs as isize);

        if nargs != 1 {
            bail!("resolve() requires 1 positional argument");
        }

        let typeobj = ObjectRef::new(*args.offset(0))?;

        let attr = if !kwnames.is_null() {
            let mut attr = HashMap::new();
            for (index, key) in TupleRef::new(ObjectRef::new(kwnames)?).iter().enumerate() {
                let key = key.as_str()?;
                let value = ObjectRef::new(*args.offset(nargs + index as isize))?;
                attr.insert(key, value);
            }
            Some(attr)
        } else {
            None
        };

        typeobj.resolve(attr);

        Ok(Object::new_none().into_ptr())
    };

    inner().restore().unwrap_or(std::ptr::null_mut())
}

#[pymodule]
fn perde(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    #[cfg(feature = "json")]
    json::import(m)?;

    let def = pyo3::ffi::PyMethodDef {
        ml_name: "resolve\0".as_ptr() as *const c_char,
        ml_meth: Some(unsafe {
            std::mem::transmute::<_PyCFunctionFastWithKeywords, PyCFunction>(resolve)
        }),
        ml_flags: pyo3::ffi::METH_FASTCALL | pyo3::ffi::METH_KEYWORDS,
        ml_doc: "".as_ptr() as *const c_char,
    };
    unsafe {
        pyo3::ffi::PyModule_AddObject(
            m.as_ptr(),
            "resolve\0".as_ptr() as *const c_char,
            pyo3::ffi::PyCFunction_New(Box::into_raw(Box::new(def)), std::ptr::null_mut()),
        )
    };

    Ok(())
}
