use crate::{
    error::{Error, Result},
    inspect::resolve_schema,
    types::{self, DictRef, Object, ObjectRef},
};
use anyhow::{bail, Context};
use derive_new::new;
use pyo3::conversion::AsPyPointer;
use pyo3::ffi::PyObject;
// use pyo3::{prelude::*, types::*};
use indexmap::IndexMap;
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
    type Err = Error;

    #[cfg_attr(feature = "perf", flame)]
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "lowercase" => Ok(StrCase::Lower),
            "UPPERCASE" => Ok(StrCase::Upper),
            "PascalCase" => Ok(StrCase::Pascal),
            "camelCase" => Ok(StrCase::Camel),
            "snake_case" => Ok(StrCase::Snake),
            "SCREAMING_SNAKE_CASE" => Ok(StrCase::ScreamingSnake),
            "kebab-case" => Ok(StrCase::Kebab),
            "SCREAMING-KEBAB-CASE" => Ok(StrCase::ScreamingKebab),
            c => erret!("Unsupported string case: {}", c),
        }
    }
}

#[derive(Clone, Debug, Default, new)]
pub struct FieldAttr {
    pub flatten: bool,
    pub rename: Option<String>,
    pub default: Option<Object>,
    pub default_factory: Option<Object>,
    pub skip: bool,
    pub skip_deserializing: bool,
}

macro_rules! extract_parse {
    ($dict:expr, $field:expr) => {
        $dict
            .get($field)
            .map(|v| {
                let s = v.as_str()?;
                s.parse()
                    .with_context(|| format!("invalid string `{}` in attribute `{}`", s, $field))
            })
            .transpose()?
    };
}

macro_rules! extract_bool {
    ($dict:expr, $field:expr) => {
        $dict
            .get($field)
            .map(|v| v.as_bool())
            .transpose()
            .with_context(|| format!("expected `bool` in attribute `{}`", $field))?
            .unwrap_or(false)
    };
}

macro_rules! extract_str {
    ($dict:expr, $field:expr) => {
        $dict
            .get($field)
            .map(|v| v.as_str().map(|v| v.to_string()))
            .transpose()
            .with_context(|| format!("expected `str` in attribute `{}`", $field))?
    };
}

macro_rules! extract {
    ($dict:expr, $field:expr) => {
        $dict.get($field)
    };
}

impl FieldAttr {
    pub fn parse(dict: &DictRef) -> Result<Self> {
        Ok(Self::new(
            extract_bool!(dict, "flatten"),
            extract_str!(dict, "rename"),
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
    pub fn parse(dict: &DictRef) -> Result<Self> {
        Ok(Self::new(
            extract_parse!(dict, "rename_all"),
            extract_str!(dict, "rename"),
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
    pub fn parse(dict: &DictRef) -> Result<Self> {
        Ok(Self::new(
            extract_parse!(dict, "rename_all"),
            extract_str!(dict, "rename"),
        ))
    }
}

impl Schema {
    pub fn resolve<'a>(ty: &'a ObjectRef, kw: *mut PyObject) -> Result<&'a Self> {
        resolve_schema(ty, kw)
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
