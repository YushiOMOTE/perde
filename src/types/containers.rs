use super::{Object, ObjectRef, Tuple};
use crate::util::*;
use pyo3::{conversion::AsPyPointer, ffi::*, PyErr, PyResult};

#[derive(Debug, Clone)]
pub struct List(Object);

impl List {
    pub fn new(len: usize) -> PyResult<Self> {
        Ok(Self(objnew!(PyList_New(len as Py_ssize_t))?))
    }

    pub fn set(&mut self, index: usize, obj: Object) {
        unsafe {
            // This API steals the pointer, so use `into_ptr`.
            PyList_SET_ITEM(self.0.as_ptr(), index as Py_ssize_t, obj.into_ptr());
        }
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Set(Object);

impl Set {
    pub fn new() -> PyResult<Self> {
        Ok(Self(objnew!(PySet_New(std::ptr::null_mut()))?))
    }

    pub fn set(&mut self, obj: Object) -> PyResult<()> {
        unsafe {
            // This API doesn't steal.
            if PySet_Add(self.0.as_ptr(), obj.as_ptr()) != 0 {
                return Err(PyErr::fetch(py()));
            }
        }
        Ok(())
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Dict(Object);

impl Dict {
    pub fn new() -> PyResult<Self> {
        Ok(Self(objnew!(PyDict_New())?))
    }

    pub fn set(&mut self, key: Object, value: Object) -> PyResult<()> {
        unsafe {
            // This API doesn't steal.
            if PyDict_SetItem(self.0.as_ptr(), key.as_ptr(), value.as_ptr()) != 0 {
                return Err(PyErr::fetch(py()));
            }
        }
        Ok(())
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Class(Object);

impl Class {
    pub fn new(obj: Object) -> Self {
        Self(obj)
    }

    pub fn construct(&self, args: Tuple) -> PyResult<Object> {
        self.0.call(args)
    }

    pub fn is_typeof(&self, p: *mut PyObject) -> bool {
        p == self.0.as_ptr()
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Enum(Object);

impl Enum {
    pub fn new(obj: Object) -> Self {
        Self(obj)
    }

    pub fn value(&self, name: &str) -> PyResult<Object> {
        self.0.get_attr(name)
    }

    pub fn is_typeof(&self, p: *mut PyObject) -> bool {
        p == self.0.as_ptr()
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}
