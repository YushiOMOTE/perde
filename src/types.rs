use crate::util::*;
use pyo3::prelude::*;

pub struct Object {
    inner: PyObject,
}

impl Object {
    pub fn new<T: ToPyObject>(value: T) -> Self {
        Self {
            inner: value.to_object(py()),
        }
    }

    pub fn null() -> Self {
        Self {
            inner: ().to_object(py()),
        }
    }

    pub fn to_pyobj(self) -> PyObject {
        self.inner
    }
}

impl From<Object> for PyObject {
    fn from(obj: Object) -> Self {
        obj.to_pyobj()
    }
}
