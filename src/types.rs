use crate::util::*;
use pyo3::{
    prelude::*,
    types::{PyDict, PyModule, PyTuple, PyType},
};
use serde::de;
use std::{collections::HashMap, str::FromStr};

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

fn collect_flatten_args(schema: &Schema) -> HashMap<String, Schema> {
    let mut args = HashMap::new();

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

pub fn has_flatten(s: &Schema) -> bool {
    s.kwargs
        .iter()
        .find(|(_, s)| s.field_attr.flatten)
        .is_some()
}

#[derive(Clone, Debug, Default)]
pub struct FieldAttr {
    flatten: bool,
}

fn parse_field_attr(attrs: &HashMap<String, PyObject>) -> PyResult<FieldAttr> {
    let mut attr = FieldAttr::default();

    attrs.iter().for_each(|(name, _val)| match name.as_ref() {
        "perde_flatten" => {
            attr.flatten = true;
        }
        _ => {}
    });

    Ok(attr)
}

#[derive(Clone, Debug, Default)]
pub struct ContainerAttr {
    rename_all: Option<StrCase>,
    rename: Option<String>,
    deny_unknown_fields: bool,
}

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
            _ => {}
        }
    }

    Ok(attr)
}
#[pyclass]
#[derive(Clone, Debug)]
pub struct Schema {
    pub cls: Py<PyType>,
    pub clsname: String,
    pub kind: TypeKind,
    pub args: Vec<Schema>,
    pub kwargs: HashMap<String, Schema>,
    pub field_attr: FieldAttr,
    pub container_attr: ContainerAttr,
    pub flatten_args: HashMap<String, Schema>,
}

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
        kwargs: HashMap<String, Schema>,
        field_attr: HashMap<String, PyObject>,
    ) -> PyResult<Self> {
        Self::new(cls, kind, args, kwargs, field_attr)
    }
}

impl Schema {
    fn new(
        cls: Py<PyType>,
        kind: &str,
        args: Vec<Schema>,
        kwargs: HashMap<String, Schema>,
        field_attr: HashMap<String, PyObject>,
    ) -> PyResult<Self> {
        let container_attr = match cls.as_ref(py()).getattr("__perde_attr__") {
            Ok(v) => v.extract()?,
            Err(_) => HashMap::new(),
        };

        let kind = kind.parse()?;
        let field_attr = parse_field_attr(&field_attr)?;
        let container_attr = parse_container_attr(&container_attr)?;

        let clsname = container_attr
            .rename
            .clone()
            .unwrap_or_else(|| cls.as_ref(py()).name().into());
        let kwargs = kwargs
            .into_iter()
            .map(|(k, v)| (convert_stringcase(&k, container_attr.rename_all), v))
            .collect();

        let mut schema = Self {
            cls,
            clsname,
            kind,
            args,
            kwargs,
            field_attr,
            container_attr,
            flatten_args: HashMap::new(),
        };

        if has_flatten(&schema) {
            schema.flatten_args = collect_flatten_args(&schema);
        }

        Ok(schema)
    }

    pub fn is_flatten(&self) -> bool {
        self.field_attr.flatten
    }

    pub fn has_flatten(&self) -> bool {
        !self.flatten_args.is_empty()
    }

    pub fn call<E>(&self, args: impl IntoPy<Py<PyTuple>>) -> Result<Object, E>
    where
        E: de::Error,
    {
        Ok(Object::new(
            self.cls.as_ref(py()).call(args, None).map_err(de)?,
        ))
    }

    pub fn call_map<E>(&self, kwargs: Vec<(PyObject, PyObject)>) -> Result<Object, E>
    where
        E: de::Error,
    {
        let dict = PyDict::from_sequence(py(), kwargs.into_py(py())).map_err(de)?;

        Ok(Object::new(
            self.cls.as_ref(py()).call((), Some(&dict)).map_err(de)?,
        ))
    }

    pub fn call_class<'a, E>(&self, map: &mut HashMap<&'a str, PyObject>) -> Result<Object, E>
    where
        E: de::Error,
    {
        let args: Result<Vec<_>, _> = self
            .kwargs
            .iter()
            .map(|(k, schema)| {
                let k: &str = k.as_ref();
                match map.remove(k) {
                    Some(v) => Ok(v),
                    None => Err(de::Error::custom(format!("missing field \"{}\"", k))),
                }
            })
            .collect();

        let args = PyTuple::new(py(), args?);
        Ok(Object::new(
            self.cls.as_ref(py()).call(args, None).map_err(de)?,
        ))
    }

    pub fn call_flatten<'a, 'b, E>(
        &self,
        flatten_args: &'a mut HashMap<&'b str, PyObject>,
    ) -> Result<Object, E>
    where
        E: de::Error,
    {
        println!("call_flatten={:?}", flatten_args);
        let kwargs: Result<Vec<_>, _> = self
            .kwargs
            .iter()
            .map(|(k, schema)| {
                if schema.is_flatten() {
                    schema
                        .call_flatten(flatten_args)
                        .map(|v| (k.to_object(py()), v.to_pyobj()))
                } else {
                    let k: &str = k.as_ref();
                    match flatten_args.remove(k) {
                        Some(v) => Ok((k.to_object(py()), v)),
                        None => Err(de::Error::custom(format!("missing field \"{}\"", k))),
                    }
                }
            })
            .collect();

        let dict = PyDict::from_sequence(py(), kwargs?.into_py(py())).map_err(de)?;

        Ok(Object::new(
            self.cls.as_ref(py()).call((), Some(&dict)).map_err(de)?,
        ))
    }

    pub fn resolve(ty: &PyAny) -> PyResult<Self> {
        Ok(match ty.getattr("__perde_schema__") {
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

    pub fn push_by_name<E>(&mut self, name: &str) -> Result<bool, E>
    where
        E: de::Error,
    {
        let cur = self.stack.last().expect("Empty schema stack");
        let map = if cur.has_flatten() {
            &cur.flatten_args
        } else {
            &cur.kwargs
        };

        let next = match map.get(name) {
            Some(next) => next,
            None => {
                if cur.container_attr.deny_unknown_fields {
                    return Err(de::Error::custom(format!("unknown field \"{}\"", name)));
                } else {
                    return Ok(false);
                }
            }
        };
        self.stack.push(next);

        Ok(true)
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
