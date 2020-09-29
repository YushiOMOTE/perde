use crate::{
    error::Convert,
    types::{FastArgs, Object},
};
use pyo3::{conversion::AsPyPointer, ffi::*, prelude::*};
use std::{collections::HashMap, os::raw::c_char};

#[macro_use]
mod error;

#[macro_use]
mod types;

#[macro_use]
mod methods;

mod format;
mod resolve;
mod schema;

mod decode;
mod encode;

pub extern "C" fn resolve(
    _self: *mut pyo3::ffi::PyObject,
    args: *const *mut pyo3::ffi::PyObject,
    nargs: Py_ssize_t,
    kwnames: *mut pyo3::ffi::PyObject,
) -> *mut pyo3::ffi::PyObject {
    let inner = || {
        let fargs = FastArgs::new(args, nargs, kwnames);

        if fargs.num_args() != 1 {
            bail!("resolve() requires 1 positional argument");
        }

        let typeobj = fargs.arg(0)?;

        let attr = if let Some(iter) = fargs.iter_kwargs()? {
            let mut attr = HashMap::new();
            for res in iter {
                let (key, value) = res?;
                attr.insert(key, value);
            }
            Some(attr)
        } else {
            None
        };

        typeobj.resolve(attr)?;

        Ok(Object::new_none().into_ptr())
    };

    inner().restore().unwrap_or(std::ptr::null_mut())
}

#[pymodule]
fn perde(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    format::import(m)?;

    method_fastcall!(resolve, "")(m);

    Ok(())
}
