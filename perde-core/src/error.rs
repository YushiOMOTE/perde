use pyo3::{type_object::PyTypeObject, PyErr, Python};
use serde::{de, ser};
use std::fmt::{self, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    msg: String,
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

        Self { msg: t.to_string() }
    }

    pub fn restore_as<T: PyTypeObject>(self) {
        let py = unsafe { Python::assume_gil_acquired() };
        let pyerr = PyErr::new::<T, _>(self.msg);
        pyerr.restore(py);
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
        write!(f, "{}", self.msg)
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

    fn context<C>(self, context: C) -> Result<T>
    where
        C: ToString;
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

    fn context<C>(self, context: C) -> Result<T>
    where
        C: ToString,
    {
        self.map_err(|e| Error {
            msg: format!("{}: {}", context.to_string(), e),
        })
    }
}
