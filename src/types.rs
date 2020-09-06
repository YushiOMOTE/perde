use crate::util::*;
use indexmap::IndexMap;
use pyo3::{
    prelude::*,
    types::{PyDict, PyModule, PyTuple, PyType},
};
use serde::{de, ser};
use std::{borrow::Cow, collections::HashMap, str::FromStr};

struct Key(Py<PyAny>);

use std::{
    cmp::{Eq, PartialEq},
    hash::{Hash, Hasher},
};

impl Hash for Key {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.as_ref(py()).hash().unwrap().hash(state);
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.0.as_ref(py()).eq(&other.0.as_ref(py()))
    }
}

impl Eq for Key {}

lazy_static::lazy_static! {
    static ref PRIMITIVES: HashMap<Key, Schema> = {
        use pyo3::types::*;

        let mut map = HashMap::new();

        macro_rules! insert {
            ($t:ty, $k:expr) => {
                let ty = py().get_type::<$t>();
                map.insert(Key(ty.as_ref().into()), Schema::primitive(ty.into(), $k));
            }
        }

        insert!(PyUnicode, TypeKind::Str);
        insert!(PyBytes, TypeKind::Bytes);
        insert!(PyBool, TypeKind::Bool);
        insert!(PyLong, TypeKind::Int);
        insert!(PyFloat, TypeKind::Float);
        insert!(PyList, TypeKind::List);
        insert!(PyDict, TypeKind::Dict);
        insert!(PyTuple, TypeKind::Tuple);
        insert!(PySet, TypeKind::List);
        insert!(PyFrozenSet, TypeKind::List);
        insert!(PyByteArray, TypeKind::ByteArray);

        map
    };
}

pub struct Object {
    inner: PyObject,
}

impl Object {
    #[cfg_attr(feature = "perf", flame)]
    pub fn new<T: ToPyObject>(value: T) -> Self {
        Self {
            inner: value.to_object(py()),
        }
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn null() -> Self {
        Self {
            inner: ().to_object(py()),
        }
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn to_pyobj(self) -> PyObject {
        self.inner
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn to_value<'a, T: FromPyObject<'a>, E>(&'a self) -> Result<T, E>
    where
        E: ser::Error,
    {
        self.inner.extract(py()).map_err(ser)
    }
}

impl From<Object> for PyObject {
    #[cfg_attr(feature = "perf", flame)]
    fn from(obj: Object) -> Self {
        obj.to_pyobj()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    /// bytes, bytearray -> deserialize_bytes
    ByteArray,
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

    #[cfg_attr(feature = "perf", flame)]
    fn from_str(s: &str) -> PyResult<Self> {
        match s {
            "bool" => Ok(Self::Bool),
            "int" => Ok(Self::Int),
            "float" => Ok(Self::Float),
            "str" => Ok(Self::Str),
            "bytes" => Ok(Self::Bytes),
            "bytearray" => Ok(Self::ByteArray),
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

#[cfg_attr(feature = "perf", flame)]
fn collect_flatten_args(schema: &Schema) -> IndexMap<String, Schema> {
    let mut args = IndexMap::new();

    for (name, subschema) in &schema.kwargs {
        if subschema.is_flatten() {
            let subargs = collect_flatten_args(subschema);
            args.extend(subargs);
        } else {
            args.insert(name.into(), subschema.clone());
        }
    }

    args
}

#[cfg_attr(feature = "perf", flame)]
pub fn has_flatten(s: &Schema) -> bool {
    s.kwargs
        .iter()
        .find(|(_, s)| s.field_attr.flatten)
        .is_some()
}

#[derive(Clone, Debug, Default)]
pub struct FieldAttr {
    flatten: bool,
    rename: Option<String>,
    default: bool,
    skip: bool,
    skip_deserializing: bool,
}

#[cfg_attr(feature = "perf", flame)]
fn parse_field_attr(attrs: &HashMap<String, PyObject>) -> PyResult<FieldAttr> {
    let mut attr = FieldAttr::default();

    for (name, val) in attrs {
        match name.as_ref() {
            "perde_flatten" => {
                attr.flatten = true;
            }
            "perde_rename" => {
                let rename: &str = val.extract(py())?;
                attr.rename = Some(rename.into());
            }
            "perde_default" => {
                attr.default = true;
            }
            "perde_skip" => {
                attr.skip = true;
            }
            "perde_skip_deserializing" => {
                attr.skip_deserializing = true;
            }
            _ => {}
        }
    }

    Ok(attr)
}

#[derive(Clone, Debug, Default)]
pub struct ContainerAttr {
    rename_all: Option<StrCase>,
    rename: Option<String>,
    deny_unknown_fields: bool,
    default: bool,
}

#[cfg_attr(feature = "perf", flame)]
fn parse_container_attr(attrs: &HashMap<String, PyObject>) -> PyResult<ContainerAttr> {
    let mut attr = ContainerAttr::default();

    for (name, val) in attrs {
        match name.as_ref() {
            "rename_all" => {
                let case: &str = val.extract(py())?;
                attr.rename_all = Some(case.parse()?);
            }
            "deny_unknown_fields" => {
                attr.deny_unknown_fields = true;
            }
            "default" => {
                attr.default = true;
            }
            _ => {}
        }
    }

    Ok(attr)
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct Schema {
    kind: TypeKind,
    args: Vec<Schema>,
    kwargs: IndexMap<String, Schema>,
    cls: Py<PyType>,
    clsname: String,
    argname: String,
    field_attr: FieldAttr,
    container_attr: ContainerAttr,
    flatten_args: IndexMap<String, Schema>,
}

#[cfg_attr(feature = "perf", flame)]
fn convert_stringcase(s: &str, case: Option<StrCase>) -> String {
    use inflections::Inflect;

    match case {
        Some(StrCase::Lower) => s.to_lower_case(),
        Some(StrCase::Upper) => s.to_upper_case(),
        Some(StrCase::Pascal) => s.to_pascal_case(),
        Some(StrCase::Camel) => s.to_camel_case(),
        Some(StrCase::Snake) => s.to_snake_case(),
        Some(StrCase::ScreamingSnake) => s.to_constant_case(),
        Some(StrCase::Kebab) => s.to_kebab_case(),
        Some(StrCase::ScreamingKebab) => s.to_kebab_case().to_upper_case(),
        None => s.into(),
    }
}

#[pymethods]
impl Schema {
    #[new]
    fn new_(
        cls: Py<PyType>,
        kind: &str,
        args: Vec<Schema>,
        kwargs: Vec<(String, Schema)>,
        field_attr: HashMap<String, PyObject>,
        container_attr: HashMap<String, PyObject>,
    ) -> PyResult<Self> {
        Self::new(
            cls,
            kind,
            args,
            kwargs,
            field_attr,
            parse_container_attr(&container_attr)?,
        )
    }
}

impl Schema {
    fn primitive(ty: &PyType, kind: TypeKind) -> Self {
        Self {
            kind,
            args: vec![],
            kwargs: IndexMap::new(),
            cls: ty.into(),
            clsname: "".into(),
            argname: "".into(),
            field_attr: FieldAttr::default(),
            container_attr: ContainerAttr::default(),
            flatten_args: IndexMap::new(),
        }
    }

    #[cfg_attr(feature = "perf", flame)]
    fn new(
        cls: Py<PyType>,
        kind: &str,
        args: Vec<Schema>,
        kwargs: Vec<(String, Schema)>,
        field_attr: HashMap<String, PyObject>,
        container_attr: ContainerAttr,
    ) -> PyResult<Self> {
        let kind = kind.parse()?;
        let field_attr = parse_field_attr(&field_attr)?;

        let clsname = container_attr
            .rename
            .clone()
            .unwrap_or_else(|| cls.as_ref(py()).name().into());
        let kwargs = kwargs
            .into_iter()
            .map(|(k, mut v)| {
                (
                    if let Some(rename) = v.field_attr.rename.as_ref() {
                        rename.into()
                    } else {
                        convert_stringcase(&k, container_attr.rename_all)
                    },
                    {
                        v.argname = k;
                        v
                    },
                )
            })
            .collect();

        let mut schema = Self {
            cls,
            clsname,
            argname: "".into(),
            kind,
            args,
            kwargs,
            field_attr,
            container_attr,
            flatten_args: IndexMap::new(),
        };

        if has_flatten(&schema) {
            schema.flatten_args = collect_flatten_args(&schema);
        }

        Ok(schema)
    }

    pub fn name(&self) -> Cow<str> {
        self.cls.as_ref(py()).name()
    }

    pub fn clsname(&self) -> &str {
        &self.clsname
    }

    pub fn kind(&self) -> TypeKind {
        self.kind
    }

    pub fn num_args(&self) -> usize {
        self.args.len()
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn is_flatten(&self) -> bool {
        self.field_attr.flatten
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn has_flatten(&self) -> bool {
        !self.flatten_args.is_empty()
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn call<E>(&self, args: impl IntoPy<Py<PyTuple>>) -> Result<Object, E>
    where
        E: de::Error,
    {
        Ok(Object::new(
            self.cls.as_ref(py()).call(args, None).map_err(de)?,
        ))
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn call_default<E>(&self) -> Result<Object, E>
    where
        E: de::Error,
    {
        Ok(Object::new(self.cls.as_ref(py()).call0().map_err(de)?))
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn call_map<E>(&self, kwargs: Vec<(PyObject, PyObject)>) -> Result<Object, E>
    where
        E: de::Error,
    {
        let dict = PyDict::from_sequence(py(), kwargs.into_py(py())).map_err(de)?;

        Ok(Object::new(
            self.cls.as_ref(py()).call((), Some(&dict)).map_err(de)?,
        ))
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn variant(&self, name: &str) -> PyResult<Option<Object>> {
        self.kwargs
            .iter()
            .find(|(n, _)| *n == name)
            .map(|_| {
                Ok(self
                    .cls
                    .as_ref(py())
                    .getattr(name)
                    .map(|v| Object::new(v))?)
            })
            .transpose()
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn variant_names(&self) -> Vec<&str> {
        self.kwargs.iter().map(|(name, _)| name.as_ref()).collect()
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn type_names(&self) -> Vec<Cow<str>> {
        self.kwargs.iter().map(|(_, s)| s.name()).collect()
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn call_class<'a, E>(&self, map: &mut HashMap<&'a str, PyObject>) -> Result<Object, E>
    where
        E: de::Error,
    {
        let args: Result<Vec<_>, E> = self
            .kwargs
            .iter()
            .map(|(k, schema)| {
                let k: &str = k.as_ref();
                match map.remove(k) {
                    Some(v) => Ok(v),
                    None => {
                        if self.container_attr.default
                            || schema.field_attr.default
                            || schema.field_attr.skip
                            || schema.field_attr.skip_deserializing
                        {
                            Ok(schema.call_default()?.to_pyobj())
                        } else {
                            Err(de::Error::custom(format!("missing field \"{}\"", k)))
                        }
                    }
                }
            })
            .collect();

        self.call_class_by_vec(args?)
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn call_class_by_vec<'a, E>(&self, args: Vec<PyObject>) -> Result<Object, E>
    where
        E: de::Error,
    {
        macro_rules! to_tuple {
            ($args:expr, $($t:tt),*) => {{
                let mut args = $args.into_iter();
                $(let $t = args.next().unwrap();)*
                    Ok(Object::new(
                        self.cls
                            .as_ref(py())
                            .call1(($($t,)*))
                            .map_err(de)?,
                    ))
            }}
        }

        match args.len() {
            0 => Ok(Object::new(self.cls.as_ref(py()).call0().map_err(de)?)),
            1 => to_tuple!(args, a),
            2 => to_tuple!(args, a, b),
            3 => to_tuple!(args, a, b, c),
            4 => to_tuple!(args, a, b, c, d),
            5 => to_tuple!(args, a, b, c, d, e),
            6 => to_tuple!(args, a, b, c, d, e, f),
            7 => to_tuple!(args, a, b, c, d, e, f, g),
            8 => to_tuple!(args, a, b, c, d, e, f, g, h),
            9 => to_tuple!(args, a, b, c, d, e, f, g, h, i),
            _ => {
                let args = PyTuple::new(py(), args);
                Ok(Object::new(self.cls.as_ref(py()).call1(args).map_err(de)?))
            }
        }
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn call_flatten<'a, 'b, E>(
        &self,
        flatten_args: &'a mut HashMap<&'b str, PyObject>,
    ) -> Result<Object, E>
    where
        E: de::Error,
    {
        let args: Result<Vec<_>, _> = self
            .kwargs
            .iter()
            .map(|(k, schema)| {
                if schema.is_flatten() {
                    schema.call_flatten(flatten_args).map(|v| v.to_pyobj())
                } else {
                    let k: &str = k.as_ref();
                    match flatten_args.remove(k) {
                        Some(v) => Ok(v),
                        None => {
                            if self.container_attr.default
                                || schema.field_attr.default
                                || schema.field_attr.skip
                                || schema.field_attr.skip_deserializing
                            {
                                Ok(schema.call_default()?.to_pyobj())
                            } else {
                                Err(de::Error::custom(format!("missing field \"{}\"", k)))
                            }
                        }
                    }
                }
            })
            .collect();

        self.call_class_by_vec(args?)
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn resolve<'a>(ty: &'a PyAny) -> PyResult<&'a PyCell<Self>> {
        Self::resolve_with_attr(ty, None)
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn resolve_primitive(ty: &PyAny) -> Option<&Self> {
        PRIMITIVES.get(&Key(ty.into()))
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn with<F, R>(ty: &PyAny, f: F) -> PyResult<R>
    where
        F: FnOnce(&Self) -> PyResult<R>,
    {
        match Self::resolve(ty) {
            Ok(ty) => {
                let ty = ty.borrow();
                f(&*ty)
            }
            Err(e) => match Self::resolve_primitive(ty) {
                Some(ty) => f(ty),
                None => Err(e),
            },
        }
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn resolve_with_attr<'a>(
        ty: &'a PyAny,
        kwargs: Option<&PyDict>,
    ) -> PyResult<&'a PyCell<Self>> {
        Ok(match ty.getattr("__perde_schema__") {
            Ok(attr) => attr.extract()?,
            Err(_) => {
                let schema = Schema::walk(ty, kwargs)?;
                ty.setattr("__perde_schema__", schema)?;
                &schema
            }
        })
    }

    #[cfg_attr(feature = "perf", flame)]
    fn walk<'a>(ty: &'a PyAny, kwargs: Option<&PyDict>) -> PyResult<&'a PyCell<Self>> {
        let module = PyModule::from_code(py(), include_str!("walk.py"), "walk.py", "walk")?;

        Ok(module
            .call("to_schema", (ty, PyDict::new(py())), kwargs)?
            .extract()?)
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn type_param(&self, index: usize) -> PyResult<&Schema> {
        self.args.get(index).ok_or_else(|| {
            pyerr(format!(
                "the type parameter {} in the type definition of `{}` is missing",
                index, self.clsname
            ))
        })
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn compatible_type_param(&self, kinds: &[TypeKind]) -> Option<&Schema> {
        self.args.iter().find(|s| kinds.contains(&s.kind))
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn member<E>(&self, name: &str) -> Result<Option<&Schema>, E>
    where
        E: de::Error,
    {
        let map = if self.has_flatten() {
            &self.flatten_args
        } else {
            &self.kwargs
        };

        map.get(name)
            .map(|v| {
                if self.field_attr.skip || self.field_attr.skip_deserializing {
                    Ok(None)
                } else {
                    Ok(Some(v))
                }
            })
            .unwrap_or_else(|| {
                if self.container_attr.deny_unknown_fields {
                    Err(de::Error::custom(format!(
                        "the member `{}` in `{}` type is missing",
                        name, self.clsname
                    )))
                } else {
                    Ok(None)
                }
            })
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn retrieve_members<'a, 'b, E>(
        &'a self,
        value: &'b PyAny,
    ) -> Result<Vec<(&'a str, &'b PyAny, &'a Schema)>, E>
    where
        E: ser::Error,
    {
        self.kwargs.iter().try_fold(vec![], |mut mems, (k, s)| {
            if s.field_attr.flatten {
                let v = value.getattr(&s.argname).map_err(ser)?;
                mems.extend(s.retrieve_members(v)?);
            } else {
                mems.push((k.as_ref(), value.getattr(&s.argname).map_err(ser)?, s))
            }
            Ok(mems)
        })
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn verify_variant<'a, E>(&self, value: &'a PyAny) -> Result<&'a str, E>
    where
        E: ser::Error,
    {
        let name: &str = value
            .getattr("name")
            .and_then(|v| v.extract())
            .map_err(ser)?;
        self.kwargs
            .get(name)
            .ok_or_else(|| ser::Error::custom(format!("unknown variant `{}`", name)))?;
        Ok(name)
    }

    #[cfg_attr(feature = "perf", flame)]
    pub fn find_union_variant<E>(&self, value: &PyAny) -> Result<&Schema, E>
    where
        E: ser::Error,
    {
        self.args
            .iter()
            .find(|s| s.cls.as_ref(py()).eq(value.get_type()))
            .ok_or_else(|| {
                ser::Error::custom(format!("unknown variant `{}`", value.get_type().name()))
            })
    }
}
