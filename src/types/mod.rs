#[macro_use]
mod object;
mod args;
mod containers;
mod tuple;

pub use self::args::*;
pub use self::containers::*;
pub use self::object::*;
pub use self::tuple::*;

use pyo3::ffi::*;
use std::{
    fmt::{self, Display},
    os::raw::c_char,
};

#[derive(Debug, Clone)]
pub struct AttrStr(String);

impl AttrStr {
    pub fn new(s: &str) -> Self {
        Self(format!("{}\0", s))
    }

    pub fn as_ptr(&self) -> *mut c_char {
        self.0.as_ptr() as *mut c_char
    }
}

impl Display for AttrStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

pub type _PyCFunctionFastWithKeywords = unsafe extern "C" fn(
    slf: *mut pyo3::ffi::PyObject,
    args: *const *mut pyo3::ffi::PyObject,
    nargs: Py_ssize_t,
    kwnames: *mut pyo3::ffi::PyObject,
) -> *mut pyo3::ffi::PyObject;
