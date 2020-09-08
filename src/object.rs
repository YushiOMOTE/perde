use crate::{schema::Schema, util::*};
use pyo3::prelude::*;

pub struct Object {
    inner: PyObject,
}

impl Object {
    #[cfg_attr(feature = "perf", flame)]
    pub fn new<T: ToPyObject>(value: T) -> Self {
        Self {
            inner: value.to_object(py()),
        }
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn null() -> Self {
        Self {
            inner: ().to_object(py()),
        }
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn to_pyobj(self) -> PyObject {
        self.inner
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn to_value<'a, T: FromPyObject<'a>>(&'a self) -> PyResult<T> {
        self.inner.extract(py())
    }
}

impl From<Object> for PyObject {
    #[cfg_attr(feature = "perf", flame)]
    fn from(obj: Object) -> Self {
        obj.to_pyobj()
    }
}

pub struct TypedObject<'a> {
    pub schema: &'a Schema,
    pub object: &'a PyAny,
}

impl<'a> TypedObject<'a> {
    pub fn new(schema: &'a Schema, object: &'a PyAny) -> Self {
        Self { schema, object }
    }
}
