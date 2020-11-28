use crate::formats::{json::*, msgpack::*, toml::*, yaml::*};
use perde_core::prelude::*;
use pyo3::ffi::*;
use std::collections::HashMap;

mod formats;

pyo3::create_exception!(perde, Error, pyo3::exceptions::PyException);

pub extern "C" fn resolve(
    _self: *mut pyo3::ffi::PyObject,
    args: *const *mut pyo3::ffi::PyObject,
    nargs: Py_ssize_t,
    kwnames: *mut pyo3::ffi::PyObject,
) -> *mut pyo3::ffi::PyObject {
    let inner = || {
        let args = FastArgs::new(args, nargs, kwnames);

        if args.num_args() != 1 {
            bail_type_err!(
                "resolve() requires 1 positional argument but got {}",
                args.num_args()
            );
        }

        let typeobj = args.arg(0)?;

        let attr = if let Some(iter) = args.iter_kwargs()? {
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

        Ok(Object::new_none())
    };

    match inner() {
        Ok(p) => p.into_ptr(),
        Err(e) => {
            e.restore_as::<Error>();
            std::ptr::null_mut()
        }
    }
}

module!(
    perde,
    submodule!(json),
    submodule!(toml),
    submodule!(yaml),
    submodule!(msgpack),
    exception!(Error),
    method_fastcall!(resolve, "resolve", "")
);
