use crate::util::*;
use pyo3::{
    prelude::*,
    types::{PyDict, PyList, PyModule, PyTuple, PyType},
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

#[derive(Clone, Debug)]
pub enum TypeKind {
    Primitive,
    Class,
    Dict,
    List,
    Tuple,
}

fn type_kind(cls: &PyType) -> PyResult<TypeKind> {
    Ok(if is_dataclass(cls)? {
        println!("{} is Class", cls.name());
        TypeKind::Class
    } else if cls.is_subclass::<PyDict>()? {
        println!("{} is Dict", cls.name());
        TypeKind::Dict
    } else if cls.is_subclass::<PyTuple>()? {
        println!("{} is Typle", cls.name());
        TypeKind::Tuple
    } else if cls.is_subclass::<PyList>()? {
        println!("{} is List", cls.name());
        TypeKind::List
    } else {
        println!("{} is Primitive", cls.name());
        TypeKind::Primitive
    })
}

fn is_dataclass(cls: &PyType) -> PyResult<bool> {
    let dataclasses = PyModule::import(py(), "dataclasses")?;
    Ok(dataclasses.call1("is_dataclass", (cls,))?.extract()?)
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct Schema {
    pub cls: Py<PyType>,
    pub args: Vec<Schema>,
    pub kwargs: HashMap<String, Schema>,
    pub attr: Vec<String>,
    pub kind: TypeKind,
}

#[pymethods]
impl Schema {
    #[new]
    fn new_(
        cls: Py<PyType>,
        args: Vec<Schema>,
        kwargs: HashMap<String, Schema>,
        attr: Vec<String>,
    ) -> PyResult<Self> {
        Self::new(cls, args, kwargs, attr)
    }
}

impl Schema {
    fn new(
        cls: Py<PyType>,
        args: Vec<Schema>,
        kwargs: HashMap<String, Schema>,
        attr: Vec<String>,
    ) -> PyResult<Self> {
        let kind = type_kind(cls.as_ref(py()))?;

        Ok(Self {
            cls,
            args,
            kwargs,
            attr,
            kind,
        })
    }

    pub fn call(
        &self,
        args: impl IntoPy<Py<PyTuple>>,
        kwargs: Option<&PyDict>,
    ) -> PyResult<Object> {
        Ok(Object::new(self.cls.as_ref(py()).call(args, kwargs)?))
    }

    pub fn resolve(ty: &PyAny) -> PyResult<Self> {
        Ok(match ty.getattr("__schema__") {
            Ok(attr) => attr.extract()?,
            Err(_) => Schema::walk(ty)?,
        })
    }

    fn walk(ty: &PyAny) -> PyResult<Self> {
        let module = PyModule::from_code(
            py(),
            r#"
from typing_inspect import get_origin, get_args
from perde import Schema
from dataclasses import dataclass, fields, is_dataclass, field
from typing import Dict, TypeVar, Union, List

def to_field(f: TypeVar):
    print(get_origin(f))
    if is_dataclass(f):
        return to_class(f)
    elif get_origin(f) is not None:
        return to_generic(f)
    else:
        return Schema(f, [], {}, [])

def to_generic(d: TypeVar):
    args = [to_field(arg) for arg in get_args(d)]
    return Schema(get_origin(d), args, {}, [])

def to_primitive(d: TypeVar):
    return Schema(d, [], {}, [])

def to_class(d: TypeVar):
    fs = dict([(f.name, to_field(f.type)) for f in fields(d)])
    return Schema(d, [], fs, [])

def to_schema(d: TypeVar):
    return to_field(d)
"#,
            "walk.py",
            "walk",
        )?;

        Ok(module.call1("to_schema", (ty,))?.extract()?)
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

    pub fn current(&self) -> &'a Schema {
        *self.stack.last().expect("Empty schema stack")
    }

    pub fn push_by_name(&mut self, name: &str) -> PyResult<()> {
        let cur = self.stack.last().expect("Empty schema stack");
        let next = cur
            .kwargs
            .get(name)
            .ok_or_else(|| pyerr(format!("Couldn't find field with name: {}", name)))?;
        self.stack.push(next);
        Ok(())
    }

    pub fn push_by_index(&mut self, index: usize) -> PyResult<()> {
        let cur = self.stack.last().expect("Empty schema stack");
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
