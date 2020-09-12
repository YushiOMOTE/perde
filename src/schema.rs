use crate::{
    inspect::resolve_schema,
    object::TypedObject,
    types::{self, Object, ObjectRef},
    util::*,
};
use derive_new::new;
use pyo3::conversion::AsPyPointer;
use pyo3::{prelude::*, types::*};
use std::str::FromStr;

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

impl Schema {
    pub fn resolve<'a>(ty: ObjectRef<'a>) -> PyResult<&'a Self> {
        resolve_schema(ty)
    }

    pub fn serialize<'a, S: serde::ser::Serializer>(value: &PyAny, serializer: S) -> PyResult<()> {
        use serde::Serialize;
        let ty = value.get_type().as_ref();
        let schema = resolve_schema(unsafe { ObjectRef::new(ty.as_ptr())? })?;
        TypedObject::new(schema, value)
            .serialize(serializer)
            .map_err(pyerr)?;
        Ok(())
    }
}

#[derive(Debug, Clone, new)]
pub struct Bytes {
    pub is_byte_array: bool,
}

#[derive(Debug, Clone)]
pub enum Primitive {
    Bool,
    Int,
    Float,
    Str,
    Bytes,
    ByteArray,
}

impl Primitive {
    pub fn name(&self) -> &str {
        match self {
            Self::Bool => "bool".into(),
            Self::Int => "int".into(),
            Self::Float => "float".into(),
            Self::Str => "str".into(),
            Self::Bytes => "bytes".into(),
            Self::ByteArray => "bytearray".into(),
        }
    }
}

#[derive(Debug, Clone, new)]
pub struct Dict {
    pub key: Box<Schema>,
    pub value: Box<Schema>,
}

impl Dict {
    pub fn name(&self) -> &str {
        "dict"
    }
}

#[derive(Debug, Clone, new)]
pub struct List {
    pub value: Box<Schema>,
}

impl List {
    pub fn name(&self) -> &str {
        "list"
    }
}

#[derive(Debug, Clone, new)]
pub struct Set {
    pub value: Box<Schema>,
}

impl Set {
    pub fn name(&self) -> &str {
        "set"
    }
}

#[derive(Debug, Clone, new)]
pub struct Tuple {
    pub args: Vec<Schema>,
}

impl Tuple {
    pub fn name(&self) -> &str {
        "tuple"
    }
}

#[derive(Debug, Clone, new)]
pub struct Enum {
    pub attr: EnumAttr,
    pub variants: IndexMap<String, VariantSchema>,
}

impl Enum {
    pub fn name(&self) -> &str {
        "enum"
    }
}

#[derive(Debug, Clone, new)]
pub struct VariantSchema {
    pub name: String,
    pub attr: VariantAttr,
    pub value: Object,
}

#[derive(Debug, Clone, new)]
pub struct Class {
    pub ty: types::Class,
    pub name: String,
    pub attr: ClassAttr,
    pub fields: IndexMap<String, FieldSchema>,
    pub flatten_fields: IndexMap<String, FieldSchema>,
}

impl Class {
    pub fn name(&self) -> &str {
        self.ty.name()
    }
}

#[derive(Debug, Clone, new)]
pub struct FieldSchema {
    pub name: String,
    pub pos: usize,
    pub attr: FieldAttr,
    pub schema: Schema,
}

#[derive(Debug, Clone, new)]
pub struct Optional {
    pub value: Box<Schema>,
}

impl Optional {
    pub fn name(&self) -> &str {
        "optional"
    }
}

#[derive(Debug, Clone, new)]
pub struct Union {
    pub variants: Vec<Schema>,
}

impl Union {
    pub fn name(&self) -> &str {
        "union"
    }
}

macro_rules! is_type {
    ($given:expr, $($type:ty),*) => {
        $(py().get_type::<$type>().eq($given))||*
    };
}

#[derive(Debug, Clone, new)]
pub struct Any;

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
    Any(Any),
}

impl Schema {
    pub fn name(&self) -> &str {
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
            Self::Any(_) => "any",
        }
    }

    pub fn type_of(&self, value: &PyAny) -> PyResult<bool> {
        let ty = value.get_type();

        let ok = match self {
            Schema::Primitive(Primitive::Bool) if is_type!(ty, PyBool) => true,
            Schema::Primitive(Primitive::Float) if is_type!(ty, PyFloat) => true,
            Schema::Primitive(Primitive::Int) if is_type!(ty, PyLong) => true,
            Schema::Primitive(Primitive::Str) if is_type!(ty, PyUnicode) => true,
            Schema::Primitive(Primitive::ByteArray) if is_type!(ty, PyByteArray) => true,
            Schema::Primitive(Primitive::Bytes) if is_type!(ty, PyBytes) => true,
            Schema::Dict(_) if is_type!(ty, PyDict) => true,
            Schema::Tuple(_) if is_type!(ty, PyTuple) => true,
            Schema::List(_) if is_type!(ty, PyList) => true,
            Schema::Set(_) if is_type!(ty, PySet) => true,
            Schema::Class(c) if c.ty.is_typeof(ty.as_ptr()) => true,
            Schema::Enum(e) if unimplemented!() => true,
            Schema::Optional(o) if o.value.type_of(ty)? => true,
            Schema::Union(u) => {
                let v: PyResult<Vec<_>> = u.variants.iter().map(|s| s.type_of(ty)).collect();
                v?.iter().any(|v| *v)
            }
            Schema::Any(_) => true,
            _ => false,
        };
        Ok(ok)
    }
}

pub struct StaticSchema {
    pub boolean: Schema,
    pub int: Schema,
    pub string: Schema,
    pub float: Schema,
    pub bytes: Schema,
    pub bytearray: Schema,
    pub dict: Schema,
    pub list: Schema,
    pub set: Schema,
}

pub fn static_schema() -> &'static StaticSchema {
    &STATIC_SCHEMA
}

lazy_static::lazy_static! {
    static ref STATIC_SCHEMA: StaticSchema = {
        StaticSchema {
            boolean: Schema::Primitive(Primitive::Bool),
            int: Schema::Primitive(Primitive::Int),
            string: Schema::Primitive(Primitive::Str),
            float: Schema::Primitive(Primitive::Float),
            bytes: Schema::Primitive(Primitive::Bytes),
            bytearray: Schema::Primitive(Primitive::ByteArray),
            dict: Schema::Dict(Dict::new(Box::new(Schema::Any(Any::new())), Box::new(Schema::Any(Any::new())))),
            list: Schema::List(List::new(Box::new(Schema::Any(Any::new())))),
            set: Schema::Set(Set::new(Box::new(Schema::Any(Any::new())))),
        }
    };
}

unsafe impl Sync for StaticSchema {}
