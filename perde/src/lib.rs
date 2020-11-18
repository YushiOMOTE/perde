use perde_core::prelude::*;
use pyo3::ffi::*;
use std::collections::HashMap;

pyo3::create_exception!(perde, Error, pyo3::exceptions::PyException);

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

module!(perde, exception!(Error), method_fastcall!(resolve, ""));
