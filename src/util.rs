use pyo3::{exceptions, prelude::*};

pub fn py<'a>() -> Python<'a> {
    unsafe { Python::assume_gil_acquired() }
}

pub fn pyerr<E: ToString>(e: E) -> PyErr {
    let py = py();

    if PyErr::occurred(py) {
        PyErr::fetch(py)
    } else {
        PyErr::new::<exceptions::RuntimeError, _>(e.to_string())
    }
}
