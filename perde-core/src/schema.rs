use crate::{
    error::{Convert, Error, Result},
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
            .and_then(|map| map.get($field).map(|v| (*v).owned()))
    };
}

#[derive(Clone, Debug, Default, PartialEq, Eq, new)]
pub struct FieldAttr {
    pub flatten: bool,
    pub rename: Option<String>,
    pub use_default: bool,
    pub default: Option<Object>,
    pub default_factory: Option<Object>,
    pub skip: bool,
    pub skip_serializing: bool,
    pub skip_deserializing: bool,
}

impl FieldAttr {
    pub fn parse(attr: &Option<&ObjectRef>) -> Result<Self> {
        Ok(Self::new(
            extract_bool!(attr, "perde_flatten"),
            extract_str!(attr, "perde_rename"),
            extract_bool!(attr, "perde_default"),
            extract!(attr, "default"),
            extract!(attr, "default_factory"),
            extract_bool!(attr, "perde_skip"),
            extract_bool!(attr, "perde_skip_serializing"),
            extract_bool!(attr, "perde_skip_deserializing"),
        ))
    }
}

#[derive(Clone, Debug, Default, new, PartialEq, Eq)]
pub struct VariantAttr {
    pub rename: Option<String>,
    pub skip: bool,
    pub skip_serializing: bool,
    pub skip_deserializing: bool,
    pub other: bool,
}

impl VariantAttr {
    pub fn parse(attr: &Option<&ObjectRef>) -> Result<Self> {
        Ok(Self::new(
            extract_str!(attr, "perde_rename"),
            extract_bool!(attr, "perde_skip"),
            extract_bool!(attr, "perde_skip_serializing"),
            extract_bool!(attr, "perde_skip_deserializing"),
            extract_bool!(attr, "perde_other"),
        ))
    }
}

#[derive(Clone, Debug, Default, new, PartialEq, Eq)]
pub struct ClassAttr {
    pub rename_all: Option<StrCase>,
    pub rename_all_serialize: Option<StrCase>,
    pub rename_all_deserialize: Option<StrCase>,
    pub rename: Option<String>,
    pub deny_unknown_fields: bool,
    pub default: bool,
}

impl ClassAttr {
    pub fn parse(attr: &Option<HashMap<&str, &ObjectRef>>) -> Result<Self> {
        Ok(Self::new(
            extract_parse!(attr, "rename_all"),
            extract_parse!(attr, "rename_all_serialize"),
            extract_parse!(attr, "rename_all_deserialize"),
            extract_str!(attr, "rename"),
            extract_bool!(attr, "deny_unknown_fields"),
            extract_bool!(attr, "default"),
        ))
    }
}

#[derive(Clone, Debug, Default, new, PartialEq, Eq)]
pub struct EnumAttr {
    pub rename_all: Option<StrCase>,
    pub rename_all_serializing: Option<StrCase>,
    pub rename_all_deserializing: Option<StrCase>,
    pub rename: Option<String>,
    pub as_value: bool,
}

impl EnumAttr {
    pub fn parse(attr: &Option<HashMap<&str, &ObjectRef>>) -> Result<Self> {
        Ok(Self::new(
            extract_parse!(attr, "rename_all"),
            extract_parse!(attr, "rename_all_serialize"),
            extract_parse!(attr, "rename_all_deserialize"),
            extract_str!(attr, "rename"),
            extract_bool!(attr, "as_value"),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct Dict {
    pub key: Box<Schema>,
    pub value: Box<Schema>,
}

impl Dict {
    pub fn name(&self) -> &str {
        "dict"
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct List {
    pub value: Box<Schema>,
}

impl List {
    pub fn name(&self) -> &str {
        "list"
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct Set {
    pub value: Box<Schema>,
}

impl Set {
    pub fn name(&self) -> &str {
        "set"
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct FrozenSet {
    pub value: Box<Schema>,
}

impl FrozenSet {
    pub fn name(&self) -> &str {
        "fronzen_set"
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct Tuple {
    pub args: Vec<Schema>,
    #[new(default)]
    pub any: bool,
}

impl Tuple {
    pub fn any_tuple() -> Self {
        Self {
            args: vec![],
            any: true,
        }
    }

    pub fn name(&self) -> &str {
        "tuple"
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct Enum {
    pub name: String,
    pub object: Object,
    pub attr: EnumAttr,
    pub variants: IndexMap<String, VariantSchema>,
}

impl Enum {
    pub fn name(&self) -> &str {
        "enum"
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct VariantSchema {
    pub name: String,
    pub attr: VariantAttr,
    pub value: Object,
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
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

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct FieldSchema {
    pub name: AttrStr,
    pub pos: usize,
    pub attr: FieldAttr,
    pub schema: Schema,
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct Optional {
    pub value: Box<Schema>,
}

impl Optional {
    pub fn name(&self) -> &str {
        "optional"
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct Union {
    pub variants: Vec<Schema>,
    #[new(default)]
    pub optional: bool,
}

impl Union {
    pub fn new_optional(variants: Vec<Schema>, optional: bool) -> Self {
        Self { variants, optional }
    }

    pub fn name(&self) -> &str {
        "union"
    }
}

pub const SCHEMA_ANY: &'static Schema = &Schema::Any(Any);

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct Any;

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub enum Schema {
    Primitive(Primitive),
    Dict(Dict),
    List(List),
    Set(Set),
    FrozenSet(FrozenSet),
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
            Self::FrozenSet(s) => s.name(),
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
    pub tuple: Schema,
    pub set: Schema,
    pub frozenset: Schema,
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
            tuple: Schema::Tuple(Tuple::any_tuple()),
            set: Schema::Set(Set::new(Box::new(Schema::Any(Any::new())))),
            frozenset: Schema::FrozenSet(FrozenSet::new(Box::new(Schema::Any(Any::new())))),
        }
    };
}

unsafe impl Sync for StaticSchema {}
