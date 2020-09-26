use anyhow::anyhow;
use pyo3::{exceptions::PyRuntimeError, PyErr, Python};
use serde::{de, ser};
use std::fmt::{self, Display};

pub use anyhow::{Error, Result};

pub trait Convert<T> {
    fn de<E>(self) -> std::result::Result<T, E>
    where
        E: de::Error,
        Self: Sized;

    fn ser<E>(self) -> std::result::Result<T, E>
    where
        E: ser::Error,
        Self: Sized;

    fn restore(self) -> Option<T>
    where
        Self: Sized;
}

impl<T> Convert<T> for Result<T> {
    fn de<E>(self) -> std::result::Result<T, E>
    where
        E: de::Error,
        Self: Sized,
    {
        self.map_err(|e| de::Error::custom(e.to_string()))
    }

    fn ser<E>(self) -> std::result::Result<T, E>
    where
        E: ser::Error,
        Self: Sized,
    {
        self.map_err(|e| ser::Error::custom(e.to_string()))
    }

    fn restore(self) -> Option<T> {
        match self {
            Ok(t) => Some(t),
            Err(e) => {
                let py = unsafe { Python::assume_gil_acquired() };
                match e.downcast::<PyErr>() {
                    Ok(pyerr) => pyerr.restore(py),
                    Err(e) => {
                        PyErr::new::<PyRuntimeError, _>(e.to_string()).restore(py);
                    }
                }
                None
            }
        }
    }
}
