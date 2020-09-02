use crate::util::*;
use pyo3::{
    prelude::*,
    types::{PyDict, PyModule, PyTuple, PyType},
};
use serde::de;
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
    /// bool -> deserialize_bool
    Bool,
    /// int -> deserialize_i64
    Int,
    /// float -> deserialize_f64
    Float,
    /// str -> deserialize_str
    Str,
    /// bytes, bytearray -> deserialize_bytes
    Bytes,
    /// list, set, frozenset -> deserialize_seq
    List,
    /// tuple -> deserialize_tuple
    Tuple,
    /// dict -> deserialize_map
    Dict,
    /// dataclass -> deserialize_map
    Class,
    /// enum.Enum -> deserialize_enum
    Enum,
    /// typing.Optional -> deserialize_option
    Option,
    /// typing.Union -> deserialize_enum
    Union,
}

impl std::str::FromStr for TypeKind {
    type Err = PyErr;

    fn from_str(s: &str) -> PyResult<Self> {
        match s {
            "bool" => Ok(Self::Bool),
            "int" => Ok(Self::Int),
            "float" => Ok(Self::Float),
            "str" => Ok(Self::Str),
            "bytes" | "bytearray" => Ok(Self::Bytes),
            "list" => Ok(Self::List),
            "tuple" => Ok(Self::Tuple),
            "dict" => Ok(Self::Dict),
            "class" => Ok(Self::Class),
            "enum" => Ok(Self::Enum),
            "option" => Ok(Self::Option),
            "union" => Ok(Self::Union),
            t => Err(pyerr(format!("Unsupported type: {}", t))),
        }
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct Schema {
    pub cls: Py<PyType>,
    pub kind: TypeKind,
    pub args: Vec<Schema>,
    pub kwargs: HashMap<String, Schema>,
    pub attr: Vec<String>,
}

#[pymethods]
impl Schema {
    #[new]
    fn new_(
        cls: Py<PyType>,
        kind: &str,
        args: Vec<Schema>,
        kwargs: HashMap<String, Schema>,
        attr: Vec<String>,
    ) -> PyResult<Self> {
        Self::new(cls, kind, args, kwargs, attr)
    }
}

impl Schema {
    fn new(
        cls: Py<PyType>,
        kind: &str,
        args: Vec<Schema>,
        kwargs: HashMap<String, Schema>,
        attr: Vec<String>,
    ) -> PyResult<Self> {
        let kind = kind.parse()?;

        Ok(Self {
            cls,
            kind,
            args,
            kwargs,
            attr,
        })
    }

    pub fn call<E>(&self, args: impl IntoPy<Py<PyTuple>>) -> Result<Object, E>
    where
        E: de::Error,
    {
        Ok(Object::new(
            self.cls.as_ref(py()).call(args, None).map_err(de)?,
        ))
    }

    pub fn call_kw<E>(&self, kwargs: Vec<(PyObject, PyObject)>) -> Result<Object, E>
    where
        E: de::Error,
    {
        let dict = PyDict::from_sequence(py(), kwargs.into_py(py())).map_err(de)?;

        Ok(Object::new(
            self.cls.as_ref(py()).call((), Some(&dict)).map_err(de)?,
        ))
    }

    pub fn resolve(ty: &PyAny) -> PyResult<Self> {
        Ok(match ty.getattr("__schema__") {
            Ok(attr) => attr.extract()?,
            Err(_) => Schema::walk(ty)?,
        })
    }

    fn walk(ty: &PyAny) -> PyResult<Self> {
        let module = PyModule::from_code(py(), include_str!("walk.py"), "walk.py", "walk")?;

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

    pub fn push_by_name<E>(&mut self, name: &str) -> Result<(), E>
    where
        E: de::Error,
    {
        let cur = self.stack.last().expect("Empty schema stack");
        let next = cur
            .kwargs
            .get(name)
            .ok_or_else(|| pyerr(format!("Couldn't find field with name: {}", name)))
            .map_err(de)?;
        self.stack.push(next);
        Ok(())
    }

    pub fn push_by_index<E>(&mut self, index: usize) -> Result<(), E>
    where
        E: de::Error,
    {
        let cur = self.stack.last().expect("Empty schema stack");
        let next = cur
            .args
            .get(index)
            .ok_or_else(|| pyerr(format!("Couldn't find field with index: {}", index)))
            .map_err(de)?;
        self.stack.push(next);
        Ok(())
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }
}
