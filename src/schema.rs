use crate::{inspect::to_schema, object::TypedObject, types, util::*};
use derive_new::new;
use indexmap::IndexMap;
use pyo3::{prelude::*, types::*};
use std::{borrow::Cow, str::FromStr};

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

impl FromStr for StrCase {
    type Err = PyErr;

    #[cfg_attr(feature = "perf", flame)]
    fn from_str(s: &str) -> PyResult<Self> {
        match s {
            "lowercase" => Ok(StrCase::Lower),
            "UPPERCASE" => Ok(StrCase::Upper),
            "PascalCase" => Ok(StrCase::Pascal),
            "camelCase" => Ok(StrCase::Camel),
            "snake_case" => Ok(StrCase::Snake),
            "SCREAMING_SNAKE_CASE" => Ok(StrCase::ScreamingSnake),
            "kebab-case" => Ok(StrCase::Kebab),
            "SCREAMING-KEBAB-CASE" => Ok(StrCase::ScreamingKebab),
            c => Err(pyerr(format!("Unsupported string case: {}", c))),
        }
    }
}

#[derive(Clone, Debug, Default, new)]
pub struct FieldAttr {
    pub flatten: bool,
    pub rename: Option<String>,
    pub default: Option<Py<PyAny>>,
    pub default_factory: Option<Py<PyAny>>,
    pub skip: bool,
    pub skip_deserializing: bool,
}

macro_rules! extract_parse {
    ($dict:expr, $field:expr) => {
        $dict
            .get_item($field)
            .map(|v| {
                let s: &str = v.extract()?;
                s.parse()
                    .map_err(|_| pyerr(format!("invalid string `{}` in attribute `{}`", s, $field)))
            })
            .transpose()?
    };
}

macro_rules! extract_bool {
    ($dict:expr, $field:expr) => {
        $dict
            .get_item($field)
            .map(|v| v.extract())
            .transpose()
            .map_err(|_| pyerr(format!("expected `bool` in attribute `{}`", $field)))?
            .unwrap_or(false)
    };
}

macro_rules! extract {
    ($dict:expr, $field:expr) => {
        $dict
            .get_item($field)
            .map(|v| v.extract())
            .transpose()
            .map_err(|_| pyerr(format!("expected `str` in attribute `{}`", $field)))?
    };
}

impl FieldAttr {
    pub fn parse(dict: &PyDict) -> PyResult<Self> {
        Ok(Self::new(
            extract_bool!(dict, "flatten"),
            extract!(dict, "rename"),
            extract!(dict, "default"),
            extract!(dict, "default_factory"),
            extract_bool!(dict, "skip"),
            extract_bool!(dict, "skip_deserializing"),
        ))
    }
}

#[derive(Clone, Debug, Default, new)]
pub struct VariantAttr {
    pub rename: Option<String>,
}

#[derive(Clone, Debug, Default, new)]
pub struct ClassAttr {
    pub rename_all: Option<StrCase>,
    pub rename: Option<String>,
    pub deny_unknown_fields: bool,
    pub default: bool,
}

impl ClassAttr {
    pub fn parse(dict: &PyDict) -> PyResult<Self> {
        Ok(Self::new(
            extract_parse!(dict, "rename_all"),
            extract!(dict, "rename"),
            extract_bool!(dict, "deny_unknown_fields"),
            extract_bool!(dict, "default"),
        ))
    }
}

#[derive(Clone, Debug, Default, new)]
pub struct EnumAttr {
    pub rename_all: Option<StrCase>,
    pub rename: Option<String>,
}

impl EnumAttr {
    pub fn parse(dict: &PyDict) -> PyResult<Self> {
        Ok(Self::new(
            extract_parse!(dict, "rename_all"),
            extract!(dict, "rename"),
        ))
    }
}

#[pyclass]
#[derive(Debug, Clone, new)]
pub struct SchemaInfo {
    pub schema: Schema,
}

impl Schema {
    pub fn deserialize<'de, D: serde::de::Deserializer<'de>>(
        ty: &PyAny,
        deserializer: D,
    ) -> PyResult<PyObject> {
        use serde::de::DeserializeSeed;
        let info = Self::resolve(ty, None)?;
        let info = info.borrow();
        info.schema.deserialize(deserializer).map_err(pyerr)
    }

    pub fn serialize<S: serde::ser::Serializer>(value: &PyAny, serializer: S) -> PyResult<()> {
        use serde::Serialize;
        let ty = value.get_type().as_ref();
        let info = Self::resolve(ty, None)?;
        let info = info.borrow();
        TypedObject::new(&info.schema, value)
            .serialize(serializer)
            .map_err(pyerr)?;
        Ok(())
    }

    pub fn resolve<'a>(ty: &'a PyAny, attr: Option<&PyDict>) -> PyResult<&'a PyCell<SchemaInfo>> {
        ty.getattr(SCHEMA_CACHE)
            .and_then(|v| v.extract())
            .or_else(|_| {
                to_schema(ty, attr).and_then(|schema| match &schema {
                    Schema::Class(_) => {
                        let schema = PyCell::new(py(), SchemaInfo::new(schema))?;
                        ty.setattr(SCHEMA_CACHE, schema)?;
                        Ok(schema)
                    }
                    _ => PyCell::new(py(), SchemaInfo::new(schema)),
                })
            })
    }
}

#[derive(Debug, Clone, new)]
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
            Self::Bytes(_) => "byte".into(),
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
    pub name: String,
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

macro_rules! is_type {
    ($given:expr, $($type:ty),*) => {
        $(py().get_type::<$type>().eq($given))||*
    };
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

    pub fn type_of(&self, value: &PyAny) -> PyResult<bool> {
        let ty = value.get_type();

        let ok = match self {
            Schema::Primitive(Primitive::Bool) if is_type!(ty, PyBool) => true,
            Schema::Primitive(Primitive::Float) if is_type!(ty, PyFloat) => true,
            Schema::Primitive(Primitive::Int) if is_type!(ty, PyLong) => true,
            Schema::Primitive(Primitive::Str) if is_type!(ty, PyUnicode) => true,
            Schema::Primitive(Primitive::Bytes(b))
                if b.is_byte_array && is_type!(ty, PyByteArray) =>
            {
                true
            }
            Schema::Primitive(Primitive::Bytes(_)) if is_type!(ty, PyBytes) => true,
            Schema::Dict(_) if is_type!(ty, PyDict) => true,
            Schema::Tuple(_) if is_type!(ty, PyTuple) => true,
            Schema::List(_) if is_type!(ty, PyList) => true,
            Schema::Set(_) if is_type!(ty, PySet) => true,
            Schema::Class(c) if ty.eq(c.ty.as_ref(py())) => true,
            Schema::Enum(e) if ty.eq(e.ty.as_ref(py())) => true,
            Schema::Optional(o) if o.value.type_of(ty)? => true,
            Schema::Union(u) => {
                let v: PyResult<Vec<_>> = u.variants.iter().map(|s| s.type_of(ty)).collect();
                v?.iter().any(|v| *v)
            }
            _ => false,
        };
        Ok(ok)
    }
}
