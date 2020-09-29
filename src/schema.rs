use crate::{
    error::{Convert, Error, Result},
    inspect::resolve_schema,
    types::{self, AttrStr, Object, ObjectRef},
};
use derive_new::new;
use indexmap::IndexMap;
use std::{collections::HashMap, str::FromStr};

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
            c => bail!("Unsupported string case: {}", c),
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
            .as_ref()
            .and_then(|map| {
                map.get($field).map(|v| {
                    let s = v.as_str()?;
                    s.parse().with_context(|| {
                        format!("invalid string `{}` in attribute `{}`", s, $field)
                    })
                })
            })
            .transpose()?
    };
}

macro_rules! extract_bool {
    ($dict:expr, $field:expr) => {
        $dict
            .as_ref()
            .and_then(|map| map.get($field).map(|v| v.as_bool()))
            .transpose()
            .with_context(|| format!("expected `bool` in attribute `{}`", $field))?
            .unwrap_or(false)
    };
}

macro_rules! extract_str {
    ($dict:expr, $field:expr) => {
        $dict
            .as_ref()
            .and_then(|map| map.get($field).map(|v| v.as_str().map(|v| v.to_string())))
            .transpose()
            .with_context(|| format!("expected `str` in attribute `{}`", $field))?
    };
}

macro_rules! extract {
    ($dict:expr, $field:expr) => {
        $dict
            .as_ref()
            .and_then(|map| map.get($field).map(|v| (*v).to_owned()))
    };
}

impl FieldAttr {
    pub fn parse(attr: &Option<&ObjectRef>) -> Result<Self> {
        Ok(Self::new(
            extract_bool!(attr, "flatten"),
            extract_str!(attr, "rename"),
            extract!(attr, "default"),
            extract!(attr, "default_factory"),
            extract_bool!(attr, "skip"),
            extract_bool!(attr, "skip_deserializing"),
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
    pub fn parse(attr: &Option<HashMap<&str, &ObjectRef>>) -> Result<Self> {
        Ok(Self::new(
            extract_parse!(attr, "rename_all"),
            extract_str!(attr, "rename"),
            extract_bool!(attr, "deny_unknown_fields"),
            extract_bool!(attr, "default"),
        ))
    }
}

#[derive(Clone, Debug, Default, new)]
pub struct EnumAttr {
    pub rename_all: Option<StrCase>,
    pub rename: Option<String>,
}

impl EnumAttr {
    pub fn parse(attr: &Option<HashMap<&str, &ObjectRef>>) -> Result<Self> {
        Ok(Self::new(
            extract_parse!(attr, "rename_all"),
            extract_str!(attr, "rename"),
        ))
    }
}

impl Schema {
    pub fn resolve<'a>(
        ty: &'a ObjectRef,
        kw: Option<HashMap<&str, &ObjectRef>>,
    ) -> Result<&'a Self> {
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
    pub name: AttrStr,
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

#[derive(new, Clone, Debug)]
pub struct WithSchema<'a> {
    pub schema: &'a Schema,
    pub object: &'a ObjectRef,
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
