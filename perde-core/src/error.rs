use pyo3::{exceptions::PyRuntimeError, PyErr, Python};
use serde::{de, ser};
use std::fmt::{self, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pyerr: PyErr,
    hints: Vec<String>,
}

#[macro_export]
macro_rules! err {
    ($($t:tt)*) => {
        $crate::error::Error::new(format!($($t)*))
    }
}

#[macro_export]
macro_rules! bail {
    ($($t:tt)*) => {
        return Err($crate::err!($($t)*));
    }
}

impl Error {
    pub fn new<T>(t: T) -> Self
    where
        T: ToString,
    {
        let py = unsafe { Python::assume_gil_acquired() };

        if PyErr::occurred(py) {
            unsafe { pyo3::ffi::PyErr_Clear() };
        }
        let pyerr = PyErr::new::<PyRuntimeError, _>(t.to_string());

        Self {
            pyerr,
            hints: vec![t.to_string()],
        }
    }
}

impl<T> From<T> for Error
where
    T: std::error::Error,
{
    fn from(e: T) -> Self {
        Self::new(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pyerr)
    }
}

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

    fn context<C>(self, context: C) -> Result<T>
    where
        C: Display + Send + Sync + 'static;

    fn with_context<C, F>(self, f: F) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
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
                e.pyerr.restore(py);
                None
            }
        }
    }

    fn context<C>(mut self, context: C) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
    {
        if let Err(e) = self.as_mut() {
            e.hints.push(context.to_string());
        }
        self
    }

    fn with_context<C, F>(mut self, f: F) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        if let Err(e) = self.as_mut() {
            e.hints.push(f().to_string());
        }
        self
    }
}
