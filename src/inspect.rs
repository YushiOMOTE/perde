use crate::{schema::*, util::*};
use indexmap::IndexMap;
use pyo3::{
    prelude::*,
    types::{
        PyBool, PyByteArray, PyBytes, PyDict, PyFloat, PyFrozenSet, PyList, PyLong, PyModule,
        PySet, PyType, PyUnicode,
    },
};

macro_rules! is_type {
    ($given:expr, $($type:ty),*) => {
        $(py().get_type::<$type>().eq($given))||*
    };
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

lazy_static::lazy_static! {
    static ref DATACLASSES: Option<Py<PyModule>> = {
        PyModule::import(py(), "dataclasses").ok().map(|v| v.into())
    };
    static ref TYPING_INSPECT: Option<Py<PyModule>> = {
        PyModule::import(py(), "typing_inspect").ok().map(|v| v.into())
    };
    static ref ENUM: Option<Py<PyAny>> = {
        PyModule::import(py(), "enum")
            .and_then(|m| m.getattr("Enum"))
            .ok()
            .map(|v| v.into())
    };
    static ref BUILTINS: Option<Py<PyModule>> = {
        PyModule::import(py(), "builtins")
            .ok()
            .map(|v| v.into())
    };
}

fn dataclasses<'a>(py: Python<'a>) -> PyResult<&'a PyModule> {
    DATACLASSES
        .as_ref()
        .ok_or_else(|| pyerr(format!("couldn't import `dataclasses`")))
        .map(|v| v.as_ref(py))
}

fn typing_inspect<'a>(py: Python<'a>) -> PyResult<&'a PyModule> {
    TYPING_INSPECT
        .as_ref()
        .ok_or_else(|| pyerr(format!("couldn't import `typing_inspect`")))
        .map(|v| v.as_ref(py))
}

fn builtins<'a>(py: Python<'a>) -> PyResult<&'a PyModule> {
    BUILTINS
        .as_ref()
        .ok_or_else(|| pyerr(format!("couldn't import python built-ins")))
        .map(|v| v.as_ref(py))
}

fn enum_class<'a>(py: Python<'a>) -> PyResult<&'a PyAny> {
    ENUM.as_ref()
        .ok_or_else(|| pyerr(format!("couldn't import python `enum.Enum`")))
        .map(|v| v.as_ref(py))
}

fn is_dataclass(ty: &PyAny) -> PyResult<bool> {
    dataclasses(py())?
        .call1("is_dataclass", (ty,))
        .and_then(|v| v.extract())
}

fn get_origin(ty: &PyAny) -> PyResult<&PyAny> {
    typing_inspect(py())?.call1("get_origin", (ty,))
}

fn get_args(ty: &PyAny) -> PyResult<Vec<&PyAny>> {
    typing_inspect(py())?
        .call1("get_args", (ty,))
        .and_then(|v| v.extract())
}

fn is_union_type(ty: &PyAny) -> PyResult<bool> {
    typing_inspect(py())?
        .call1("is_union_type", (ty,))
        .and_then(|v| v.extract())
}

fn is_optional_type(ty: &PyAny) -> PyResult<bool> {
    typing_inspect(py())?
        .call1("is_optional_type", (ty,))
        .and_then(|v| v.extract())
}

fn is_tuple_type(ty: &PyAny) -> PyResult<bool> {
    typing_inspect(py())?
        .call1("is_tuple_type", (ty,))
        .and_then(|v| v.extract())
}

fn is_generic(ty: &PyAny) -> PyResult<bool> {
    Ok(!get_origin(ty)?.is_none()
        && (is_union_type(ty)? || is_optional_type(ty)? || is_tuple_type(ty)?))
}

fn is_enum(ty: &PyAny) -> PyResult<bool> {
    builtins(py())?
        .call1("issubclass", (ty, enum_class(py())?))
        .and_then(|v| v.extract())
}

fn fields(ty: &PyAny) -> PyResult<Vec<&PyAny>> {
    dataclasses(py())?
        .call1("fields", (ty,))
        .and_then(|v| v.extract())
}

pub fn to_schema(ty: &PyAny, cattr: Option<&PyDict>) -> PyResult<Schema> {
    if is_dataclass(ty)? {
        to_class(ty, cattr)
    } else if is_generic(ty)? {
        to_generic(ty)
    } else if is_enum(ty)? {
        to_enum(ty, cattr)
    } else {
        to_primitive(ty)
    }
}

fn to_primitive(ty: &PyAny) -> PyResult<Schema> {
    let ty = ty.downcast()?;
    if is_type!(ty, PyUnicode) {
        Ok(Schema::Primitive(Primitive::Str))
    } else if is_type!(ty, PyBytes) {
        Ok(Schema::Primitive(Primitive::Bytes(Bytes::new(
            ty.into(),
            false,
        ))))
    } else if is_type!(ty, PyBool) {
        Ok(Schema::Primitive(Primitive::Bool))
    } else if is_type!(ty, PyLong) {
        Ok(Schema::Primitive(Primitive::Int))
    } else if is_type!(ty, PyFloat) {
        Ok(Schema::Primitive(Primitive::Float))
    } else if is_type!(ty, PyByteArray) {
        Ok(Schema::Primitive(Primitive::Bytes(Bytes::new(
            ty.into(),
            true,
        ))))
    } else {
        Err(pyerr(format!("unsupported type object `{}`", ty.name())))
    }
}

fn to_class(ty: &PyAny, cattr: Option<&PyDict>) -> PyResult<Schema> {
    let cattr = cattr
        .map(|v| ClassAttr::parse(v))
        .transpose()?
        .unwrap_or_default();
    let fields: PyResult<IndexMap<_, _>> = fields(ty)?
        .into_iter()
        .map(|v| {
            let name: &str = v.getattr("name")?.extract()?;
            let value = v.getattr("value")?;
            let metadata = v.getattr("metadata")?.extract()?;
            let attr = FieldAttr::parse(metadata)?;
            let schema = to_schema(value.get_type(), None)?;

            let origname = name.to_string();
            let name = convert_stringcase(name, cattr.rename_all);
            let name = if let Some(rename) = attr.rename.as_ref() {
                rename.into()
            } else {
                name
            };

            Ok((name, FieldSchema::new(origname, attr, schema)))
        })
        .collect();

    let ty = ty.downcast().map(|v: &PyType| v)?;
    let name = if let Some(rename) = cattr.rename.as_ref() {
        rename.into()
    } else {
        ty.name().to_string()
    };

    Ok(Schema::Class(Class::new(
        ty.into(),
        name,
        cattr,
        fields?,
        IndexMap::new(),
    )))
}

fn to_generic(ty: &PyAny) -> PyResult<Schema> {
    if is_optional_type(ty)? {
        let args = get_args(ty)?;
        let arg = args
            .get(0)
            .ok_or_else(|| pyerr("`Optional` is missing type parameter"))?;
        Ok(Schema::Optional(Optional::new(Box::new(to_schema(
            arg, None,
        )?))))
    } else if is_union_type(ty)? {
        let args: PyResult<Vec<_>> = get_args(ty)?
            .into_iter()
            .map(|v| to_schema(v, None))
            .collect();
        Ok(Schema::Union(Union::new(args?)))
    } else if is_tuple_type(ty)? {
        let args: PyResult<Vec<_>> = get_args(ty)?
            .into_iter()
            .map(|v| to_schema(v, None))
            .collect();
        Ok(Schema::Tuple(Tuple::new(args?)))
    } else {
        let origty: &PyType = get_origin(ty)?.downcast()?;

        if is_type!(origty, PyDict) {
            let args = get_args(ty)?;
            let key = args
                .get(0)
                .ok_or_else(|| pyerr("`Dict` is missing type parameter"))?;
            let value = args
                .get(1)
                .ok_or_else(|| pyerr("`Dict` is missing second type parameter"))?;
            Ok(Schema::Dict(Dict::new(
                Box::new(to_schema(key, None)?),
                Box::new(to_schema(value, None)?),
            )))
        } else if is_type!(origty, PyList) {
            let args = get_args(ty)?;
            let arg = args
                .get(0)
                .ok_or_else(|| pyerr("`List` is missing type parameter"))?;
            Ok(Schema::List(List::new(Box::new(to_schema(arg, None)?))))
        } else if is_type!(origty, PySet) || is_type!(origty, PyFrozenSet) {
            let args = get_args(ty)?;
            let arg = args
                .get(0)
                .ok_or_else(|| pyerr("`Set` is missing type parameter"))?;
            Ok(Schema::Set(Set::new(
                origty.into(),
                Box::new(to_schema(arg, None)?),
            )))
        } else {
            Err(pyerr(format!("Unsupported type `{}`", origty.name())))
        }
    }
}

fn to_enum(ty: &PyAny, cattr: Option<&PyDict>) -> PyResult<Schema> {
    let variants: PyResult<IndexMap<_, _>> = ty
        .iter()?
        .map(|v| {
            let v = v?;
            let name: &str = v.getattr("name")?.extract()?;
            let value = v.getattr("value")?;
            Ok((
                name.into(),
                VariantSchema::new(
                    name.into(),
                    VariantAttr::default(),
                    to_schema(value.get_type(), None)?,
                ),
            ))
        })
        .collect();

    let attr = cattr.map(|v| EnumAttr::parse(v)).transpose()?;

    Ok(Schema::Enum(Enum::new(
        ty.downcast::<PyType>()?.into(),
        attr.unwrap_or_default(),
        variants?,
    )))
}
