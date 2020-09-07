use crate::util::*;
use derive_new::new;
use indexmap::IndexMap;
use pyo3::{prelude::*, types::*};
use std::borrow::Cow;

const SCHEMA_CACHE: &'static str = "__perde_schema__";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StrCase {
    Lower,
    Upper,
    Pascal,
    Camel,
    Snake,
    ScreamingSnake,
    Kebab,
    ScreamingKebab,
}

#[derive(Clone, Debug, new)]
pub struct FieldAttr {
    pub flatten: bool,
    pub rename: Option<String>,
    pub default: Option<Py<PyAny>>,
    pub default_factory: Option<Py<PyAny>>,
    pub skip: bool,
    pub skip_deserializing: bool,
}

#[derive(Clone, Debug, new)]
pub struct VariantAttr {
    pub rename: Option<String>,
}

#[derive(Clone, Debug, new)]
pub struct ClassAttr {
    pub rename_all: Option<StrCase>,
    pub rename: Option<String>,
    pub deny_unknown_fields: bool,
    pub default: bool,
}

#[derive(Clone, Debug, new)]
pub struct EnumAttr {
    pub rename_all: Option<StrCase>,
    pub rename: Option<String>,
}

#[pyclass]
#[derive(Debug, Clone, new)]
struct SchemaInfo {
    pub schema: Schema,
}

impl Schema {
    fn resolve(ty: &PyAny) -> PyResult<&PyCell<SchemaInfo>> {
        ty.getattr(SCHEMA_CACHE)?.extract().or_else(|_| {
            Self::inspect(ty).and_then(|schema| {
                ty.setattr(SCHEMA_CACHE, schema)?;
                Ok(schema)
            })
        })
    }

    fn inspect(ty: &PyAny) -> PyResult<&PyCell<SchemaInfo>> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub struct Bytes {
    pub ty: Py<PyType>,
    pub is_byte_array: bool,
}

#[derive(Debug, Clone)]
pub enum Primitive {
    Bool,
    Int,
    Float,
    Str,
    Bytes(Bytes),
}

impl Primitive {
    pub fn name(&self) -> Cow<str> {
        match self {
            Self::Bool => "bool".into(),
            Self::Int => "int".into(),
            Self::Float => "float".into(),
            Self::Str => "str".into(),
            Self::Bytes(b) if b.is_byte_array => "bytearray".into(),
            Self::Bytes(b) => "byte".into(),
        }
    }
}

#[derive(Debug, Clone, new)]
pub struct Dict {
    pub key: Box<Schema>,
    pub value: Box<Schema>,
}

impl Dict {
    pub fn name(&self) -> Cow<str> {
        "dict".into()
    }
}

#[derive(Debug, Clone, new)]
pub struct List {
    pub value: Box<Schema>,
}

impl List {
    pub fn name(&self) -> Cow<str> {
        "list".into()
    }
}

#[derive(Debug, Clone, new)]
pub struct Set {
    pub ty: Py<PyType>,
    pub value: Box<Schema>,
}

impl Set {
    pub fn name(&self) -> Cow<str> {
        "set".into()
    }
}

#[derive(Debug, Clone, new)]
pub struct Tuple {
    pub args: Vec<Schema>,
}

impl Tuple {
    pub fn name(&self) -> Cow<str> {
        "tuple".into()
    }
}

#[derive(Debug, Clone, new)]
pub struct Enum {
    pub ty: Py<PyType>,
    pub attr: EnumAttr,
    pub variants: IndexMap<String, VariantSchema>,
}

impl Enum {
    pub fn name(&self) -> Cow<str> {
        self.ty.as_ref(py()).name()
    }
}

#[derive(Debug, Clone, new)]
pub struct VariantSchema {
    pub name: String,
    pub attr: VariantAttr,
    pub schema: Schema,
}

#[derive(Debug, Clone, new)]
pub struct Class {
    pub ty: Py<PyType>,
    pub attr: ClassAttr,
    pub fields: IndexMap<String, FieldSchema>,
    pub flatten_fields: IndexMap<String, FieldSchema>,
}

impl Class {
    pub fn name(&self) -> Cow<str> {
        self.ty.as_ref(py()).name()
    }
}

#[derive(Debug, Clone, new)]
pub struct FieldSchema {
    pub name: String,
    pub attr: FieldAttr,
    pub schema: Schema,
}

#[derive(Debug, Clone, new)]
pub struct Optional {
    pub value: Box<Schema>,
}

impl Optional {
    pub fn name(&self) -> Cow<str> {
        "optional".into()
    }
}

#[derive(Debug, Clone, new)]
pub struct Union {
    pub variants: Vec<Schema>,
}

impl Union {
    pub fn name(&self) -> Cow<str> {
        "union".into()
    }
}

#[derive(Debug, Clone, new)]
pub enum Schema {
    Primitive(Primitive),
    Dict(Dict),
    List(List),
    Set(Set),
    Tuple(Tuple),
    Class(Class),
    Enum(Enum),
    Optional(Optional),
    Union(Union),
}

impl Schema {
    pub fn name(&self) -> Cow<str> {
        match self {
            Self::Primitive(p) => p.name(),
            Self::Dict(d) => d.name(),
            Self::List(l) => l.name(),
            Self::Set(s) => s.name(),
            Self::Tuple(t) => t.name(),
            Self::Class(c) => c.name(),
            Self::Enum(e) => e.name(),
            Self::Optional(o) => o.name(),
            Self::Union(u) => u.name(),
        }
    }
}
