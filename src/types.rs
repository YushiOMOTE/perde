use crate::util::*;
use pyo3::ffi::*;
use pyo3::{PyErr, PyResult};
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::NonNull;

pub struct Object(NonNull<PyObject>);

impl Object {
    fn new(p: *mut PyObject) -> PyResult<Self> {
        match NonNull::new(p) {
            Some(p) => Ok(Self(p)),
            None => Err(PyErr::fetch(py())),
        }
    }

    fn as_ptr(&self) -> *mut PyObject {
        self.0.as_ptr()
    }
}

macro_rules! obj {
    ($p:expr) => {
        Object::new(unsafe { $p })
    };
}

pub fn py_none() -> PyResult<Object> {
    obj!(Py_None())
}

pub fn py_true() -> PyResult<Object> {
    obj!(Py_True())
}

pub fn py_false() -> PyResult<Object> {
    obj!(Py_False())
}

pub fn py_i64(value: i64) -> PyResult<Object> {
    obj!(PyLong_FromLongLong(value))
}

pub fn py_u64(value: u64) -> PyResult<Object> {
    obj!(PyLong_FromUnsignedLongLong(value))
}

pub fn py_f64(value: f64) -> PyResult<Object> {
    obj!(PyFloat_FromDouble(value))
}

pub fn py_str(value: &str) -> PyResult<Object> {
    obj!(PyUnicode_FromStringAndSize(
        value.as_ptr() as *const c_char,
        value.len() as Py_ssize_t
    ))
}

pub fn py_bytes(value: &[u8]) -> PyResult<Object> {
    obj!(PyBytes_FromStringAndSize(
        value.as_ptr() as *const c_char,
        value.len() as Py_ssize_t
    ))
}

pub fn py_bytearray(value: &[u8]) -> PyResult<Object> {
    obj!(PyByteArray_FromStringAndSize(
        value.as_ptr() as *const c_char,
        value.len() as Py_ssize_t
    ))
}

pub struct List(Object);

impl List {
    pub fn new(len: usize) -> PyResult<Self> {
        Ok(Self(obj!(PyList_New(len as Py_ssize_t))?))
    }

    pub fn set(&mut self, index: usize, obj: Object) {
        unsafe {
            PyList_SET_ITEM(self.0.as_ptr(), index as Py_ssize_t, obj.as_ptr());
        }
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

pub struct Set(Object);

impl Set {
    pub fn new(len: usize) -> PyResult<Self> {
        Ok(Self(obj!(PySet_New(std::ptr::null_mut()))?))
    }

    pub fn set(&mut self, obj: Object) -> PyResult<()> {
        unsafe {
            if PySet_Add(self.0.as_ptr(), obj.as_ptr()) == -1 {
                return Err(PyErr::fetch(py()));
            }
        }
        Ok(())
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

pub struct Dict(Object);

impl Dict {
    pub fn new(len: usize) -> PyResult<Self> {
        Ok(Self(obj!(PyDict_New())?))
    }

    pub fn set(&mut self, key: Object, value: Object) -> PyResult<()> {
        unsafe {
            if PyDict_SetItem(self.0.as_ptr(), key.as_ptr(), value.as_ptr()) == 1 {
                return Err(PyErr::fetch(py()));
            }
            Py_DECREF(key.as_ptr());
            Py_DECREF(value.as_ptr());
        }
        Ok(())
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

pub struct Tuple(Object);

impl Tuple {
    pub fn new(len: usize) -> PyResult<Self> {
        Ok(Self(obj!(PyTuple_New(len as Py_ssize_t))?))
    }

    pub fn set(&mut self, index: usize, obj: Object) {
        unsafe {
            PyTuple_SET_ITEM(self.0.as_ptr(), index as Py_ssize_t, obj.as_ptr());
        }
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

pub struct Class(Object);

impl Class {
    pub fn new(obj: Object) -> Self {
        Self(obj)
    }

    pub fn construct(&self, args: Tuple) -> PyResult<Object> {
        let p = args.into_inner();
        let r = obj!(PyObject_Call(
            self.0.as_ptr(),
            p.as_ptr(),
            std::ptr::null_mut()
        ))?;
        unsafe {
            Py_DECREF(p.as_ptr());
        }
        Ok(r)
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

pub struct Enum(Object);

impl Enum {
    pub fn new(obj: Object) -> Self {
        Self(obj)
    }

    pub fn value(&self, name: &str) -> PyResult<Object> {
        let s = CString::new(name).map_err(pyerr)?;
        obj!(PyObject_GetAttrString(self.0.as_ptr(), s.as_ptr()))
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}
