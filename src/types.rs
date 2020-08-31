use crate::util::*;
use pyo3::{
    prelude::*,
    types::{PyDict, PyTuple, PyType},
};
use std::collections::HashMap;

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

#[pyclass]
#[derive(Clone, Debug)]
pub struct Schema {
    pub cls: Py<PyType>,
    pub args: Vec<Schema>,
    pub kwargs: HashMap<String, Schema>,
    pub attr: Vec<String>,
}

#[pymethods]
impl Schema {
    #[new]
    fn new(
        cls: Py<PyType>,
        args: Vec<Schema>,
        kwargs: HashMap<String, Schema>,
        attr: Vec<String>,
    ) -> Self {
        Self {
            cls,
            args,
            kwargs,
            attr,
        }
    }
}

impl Schema {
    pub fn call(
        &self,
        args: impl IntoPy<Py<PyTuple>>,
        kwargs: Option<&PyDict>,
    ) -> PyResult<Object> {
        Ok(Object::new(self.cls.as_ref(py()).call(args, kwargs)?))
    }
}

#[derive(Debug)]
pub struct SchemaStack<'a> {
    stack: Vec<&'a Schema>,
}

impl<'a> SchemaStack<'a> {
    pub fn new(schema: &'a Schema) -> Self {
        Self {
            stack: vec![schema],
        }
    }

    pub fn current(&self) -> PyResult<&'a Schema> {
        self.stack
            .last()
            .map(|p| *p)
            .ok_or_else(|| pyerr("Schema stack is empty"))
    }

    pub fn push_by_name(&mut self, name: &str) -> PyResult<()> {
        let cur = self
            .stack
            .last()
            .ok_or_else(|| pyerr("Schema stack is empty"))?;
        let next = cur
            .kwargs
            .get(name)
            .ok_or_else(|| pyerr(format!("Couldn't find field with name: {}", name)))?;
        self.stack.push(next);
        Ok(())
    }

    pub fn push_by_index(&mut self, index: usize) -> PyResult<()> {
        let cur = self
            .stack
            .last()
            .ok_or_else(|| pyerr("Schema stack is empty"))?;
        let next = cur
            .args
            .get(index)
            .ok_or_else(|| pyerr(format!("Couldn't find field with index: {}", index)))?;
        self.stack.push(next);
        Ok(())
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }
}
