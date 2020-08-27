use crate::types::Object;
use pyo3::{exceptions, prelude::*};
use serde::de::{self, Error};

pub fn py<'a>() -> Python<'a> {
    unsafe { Python::assume_gil_acquired() }
}

pub fn restore<T: Error>(e: PyErr) -> T {
    e.restore(py());
    Error::custom("python error")
}

pub fn restore_fin<T: ToString>(e: T) -> Object {
    if !PyErr::occurred(py()) {
        let e = exceptions::RuntimeError::py_err(e.to_string());
        e.restore(py());
    }
    Object::null()
}
