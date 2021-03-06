use crate::{
    attr::AttrStr,
    error::{Convert, Error, Result},
    object::{Object, ObjectRef, SyncObject},
};
use derive_new::new;
use indexmap::IndexMap;
use std::{borrow::Cow, collections::HashMap, str::FromStr};

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
            c => bail_value_err!("invalid string case: `{}`", c),
        }
    }
}

macro_rules! field_extract_bool {
    ($dict:expr, $field:expr) => {
        $dict
            .as_ref()
            .and_then(|map| map.get($field).ok().map(|v| v.as_bool()))
            .transpose()
            .context(format!("invalid attribute `{}`", $field))?
            .unwrap_or(false)
    };
}

macro_rules! field_extract_str {
    ($dict:expr, $field:expr) => {
        $dict
            .as_ref()
            .and_then(|map| {
                map.get($field)
                    .ok()
                    .map(|v| v.as_str().map(|v| v.to_string()))
            })
            .transpose()
            .context(format!("invalid attribute `{}`", $field))?
    };
}

macro_rules! extract_stringcase {
    ($dict:expr, $field:expr) => {
        $dict
            .as_ref()
            .and_then(|map| {
                map.get($field).map(|v| {
                    let s = v.as_str()?;
                    s.parse()
                })
            })
            .transpose()
            .context(format!("invalid attribute `{}`", $field))?
    };
}

macro_rules! extract_bool {
    ($dict:expr, $field:expr) => {
        $dict
            .as_ref()
            .and_then(|map| map.get($field).map(|v| v.as_bool()))
            .transpose()
            .context(format!("invalid attribute `{}`", $field))?
            .unwrap_or(false)
    };
}

macro_rules! extract_str {
    ($dict:expr, $field:expr) => {
        $dict
            .as_ref()
            .and_then(|map| map.get($field).map(|v| v.as_str().map(|v| v.to_string())))
            .transpose()
            .context(format!("invalid attribute `{}`", $field))?
    };
}

#[allow(clippy::too_many_arguments)]
#[derive(Clone, Debug, Default, PartialEq, Eq, new)]
pub struct FieldAttr {
    pub flatten: bool,
    pub rename: Option<String>,
    pub default: Option<SyncObject>,
    pub default_factory: Option<SyncObject>,
    pub skip: bool,
    pub skip_serializing: bool,
    pub skip_deserializing: bool,
    pub default_construct: bool,
}

impl FieldAttr {
    pub fn parse(
        attr: Option<Object>,
        default: Option<Object>,
        default_factory: Option<Object>,
    ) -> Result<Self> {
        Ok(Self::new(
            field_extract_bool!(attr, "perde_flatten"),
            field_extract_str!(attr, "perde_rename"),
            default.map(|o| o.into()),
            default_factory.map(|o| o.into()),
            field_extract_bool!(attr, "perde_skip"),
            field_extract_bool!(attr, "perde_skip_serializing"),
            field_extract_bool!(attr, "perde_skip_deserializing"),
            field_extract_bool!(attr, "perde_default"),
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
            field_extract_str!(attr, "perde_rename"),
            field_extract_bool!(attr, "perde_skip"),
            field_extract_bool!(attr, "perde_skip_serializing"),
            field_extract_bool!(attr, "perde_skip_deserializing"),
            field_extract_bool!(attr, "perde_other"),
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
            extract_stringcase!(attr, "rename_all"),
            extract_stringcase!(attr, "rename_all_serialize"),
            extract_stringcase!(attr, "rename_all_deserialize"),
            extract_str!(attr, "rename"),
            extract_bool!(attr, "deny_unknown_fields"),
            extract_bool!(attr, "default"),
        ))
    }
}

#[derive(Clone, Debug, Default, new, PartialEq, Eq)]
pub struct EnumAttr {
    pub rename_all: Option<StrCase>,
    pub rename_all_serialize: Option<StrCase>,
    pub rename_all_deserialize: Option<StrCase>,
    pub rename: Option<String>,
    pub as_value: bool,
}

impl EnumAttr {
    pub fn parse(attr: &Option<HashMap<&str, &ObjectRef>>) -> Result<Self> {
        Ok(Self::new(
            extract_stringcase!(attr, "rename_all"),
            extract_stringcase!(attr, "rename_all_serialize"),
            extract_stringcase!(attr, "rename_all_deserialize"),
            extract_str!(attr, "rename"),
            extract_bool!(attr, "as_value"),
        ))
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
    pub object: SyncObject,
    pub attr: EnumAttr,
    pub variants: Vec<VariantSchema>,
}

impl Enum {
    pub fn name(&self) -> &str {
        "enum"
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct VariantSchema {
    pub name: String,
    pub sername: String,
    pub dename: String,
    pub attr: VariantAttr,
    pub value: SyncObject,
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct Class {
    pub ty: SyncObject,
    pub name: String,
    pub attr: ClassAttr,
    pub fields: IndexMap<String, FieldSchema>,
    pub flatten_fields: IndexMap<String, FieldSchema>,
    pub flatten_dict: Option<Dict>,
    pub ser_field_len: usize,
}

impl Class {
    pub fn name(&self) -> &str {
        self.ty.name()
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct FieldSchema {
    pub name: AttrStr,
    pub rename: String,
    pub pos: usize,
    pub attr: FieldAttr,
    pub schema: Schema,
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct Union {
    pub variants: Vec<Schema>,
    pub optional: bool,
}

impl Union {
    pub fn name(&self) -> &str {
        "union"
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct Any;

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub enum Schema {
    Bool,
    Int,
    Float,
    Str,
    Bytes,
    ByteArray,
    DateTime,
    Date,
    Time,
    Decimal,
    Uuid,
    Dict(Dict),
    List(List),
    Set(Set),
    FrozenSet(FrozenSet),
    Tuple(Tuple),
    Class(Class),
    Enum(Enum),
    Union(Union),
    Any(Any),
}

impl Schema {
    pub fn name(&self) -> &str {
        match self {
            Self::Bool => "bool",
            Self::Int => "int",
            Self::Float => "float",
            Self::Str => "str",
            Self::Bytes => "bytes",
            Self::ByteArray => "bytearray",
            Self::DateTime => "datetime",
            Self::Date => "date",
            Self::Time => "time",
            Self::Decimal => "Decimal",
            Self::Uuid => "Uuid",
            Self::Dict(d) => d.name(),
            Self::List(l) => l.name(),
            Self::Set(s) => s.name(),
            Self::FrozenSet(s) => s.name(),
            Self::Tuple(t) => t.name(),
            Self::Class(c) => c.name(),
            Self::Enum(e) => e.name(),
            Self::Union(u) => u.name(),
            Self::Any(_) => "any",
        }
    }

    pub fn is_optional(&self) -> bool {
        match self {
            Self::Union(u) if u.optional => true,
            Self::Any(_) => true,
            _ => false,
        }
    }

    pub fn borrowed(&self) -> Cow<'_, Self> {
        Cow::Borrowed(self)
    }

    pub fn owned(&self) -> Cow<'static, Self> {
        Cow::Owned(self.clone())
    }
}

#[derive(new, Clone, Debug)]
pub struct WithSchema<'a> {
    pub schema: Cow<'a, Schema>,
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
    pub datetime: Schema,
    pub date: Schema,
    pub time: Schema,
    pub decimal: Schema,
    pub uuid: Schema,
    pub any: Schema,
}

pub fn static_schema() -> &'static StaticSchema {
    &STATIC_SCHEMA
}

lazy_static::lazy_static! {
    static ref STATIC_SCHEMA: StaticSchema = {
        StaticSchema {
            boolean: Schema::Bool,
            int: Schema::Int,
            string: Schema::Str,
            float: Schema::Float,
            bytes: Schema::Bytes,
            bytearray: Schema::ByteArray,
            dict: Schema::Dict(Dict::new(Box::new(Schema::Any(Any::new())), Box::new(Schema::Any(Any::new())))),
            list: Schema::List(List::new(Box::new(Schema::Any(Any::new())))),
            tuple: Schema::Tuple(Tuple::any_tuple()),
            set: Schema::Set(Set::new(Box::new(Schema::Any(Any::new())))),
            frozenset: Schema::FrozenSet(FrozenSet::new(Box::new(Schema::Any(Any::new())))),
            datetime: Schema::DateTime,
            time: Schema::Time,
            date: Schema::Date,
            decimal: Schema::Decimal,
            uuid: Schema::Uuid,
            any: Schema::Any(Any),
        }
    };
}
