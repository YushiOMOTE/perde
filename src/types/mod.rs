#[macro_use]
mod object;
// mod containers;
mod tuple;

// pub use self::containers::*;
pub use self::object::*;
pub use self::tuple::*;

use pyo3::ffi::*;

pub type _PyCFunctionFastWithKeywords = unsafe extern "C" fn(
    slf: *mut pyo3::ffi::PyObject,
    args: *const *mut pyo3::ffi::PyObject,
    nargs: Py_ssize_t,
    kwnames: *mut pyo3::ffi::PyObject,
) -> *mut pyo3::ffi::PyObject;
