use pyo3::{type_object::PyTypeObject, PyErr, Python};
use serde::{de, ser};
use std::fmt::{self, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TypeError(String),
    Else(String),
}

#[macro_export]
macro_rules! err {
    ($($t:tt)*) => {
        $crate::error::Error::new(format!($($t)*))
    }
}

#[macro_export]
macro_rules! type_err {
    ($($t:tt)*) => {
        $crate::error::Error::type_error(format!($($t)*))
    }
}

#[macro_export]
macro_rules! bail {
    ($($t:tt)*) => {
        return Err($crate::err!($($t)*));
    }
}

#[macro_export]
macro_rules! bail_type_err {
    ($($t:tt)*) => {
        return Err($crate::type_err!($($t)*))
    }
}

pub fn raise<T: PyTypeObject, U: ToString>(msg: U) {
    let py = unsafe { Python::assume_gil_acquired() };
    let pyerr = PyErr::new::<T, _>(msg.to_string());
    pyerr.restore(py);
}

fn clear() {
    if unsafe { !pyo3::ffi::PyErr_Occurred().is_null() } {
        unsafe { pyo3::ffi::PyErr_Clear() };
    }
}

impl Error {
    pub fn new<T>(t: T) -> Self
    where
        T: ToString,
    {
        clear();
        Self::Else(t.to_string())
    }

    pub fn type_error<T>(t: T) -> Self
    where
        T: ToString,
    {
        clear();
        Self::TypeError(t.to_string())
    }

    pub fn restore_as<T: PyTypeObject>(self) {
        match self {
            Error::TypeError(t) => raise::<pyo3::exceptions::PyTypeError, _>(t),
            Error::Else(t) => raise::<T, _>(t),
        }
    }

    pub fn message(&self) -> &str {
        match self {
            Self::TypeError(m) | Self::Else(m) => &m,
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
        write!(
            f,
            "{}",
            match self {
                Error::TypeError(s) | Error::Else(s) => s,
            }
        )
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
        self.map_err(|mut e| {
            let new_msg = format!("{}: {}", context.to_string(), e);
            match &mut e {
                Error::TypeError(m) | Error::Else(m) => {
                    *m = new_msg;
                }
            };
            e
        })
    }
}
