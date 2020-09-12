use crate::{
    schema::*,
    types::{self, obj_none_type, Object, ObjectRef, StaticObject, TupleRef},
    util::*,
};
use pyo3::{conversion::AsPyPointer, ffi::*, types::PyModule, PyResult};
use std::os::raw::c_char;

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

macro_rules! ptr {
    ($p:expr) => {
        unsafe { &mut $p as *mut _ as *mut PyObject }
    };
}

macro_rules! is {
    ($e:expr, $t:expr) => {
        unsafe { $e as *mut PyTypeObject == &mut $t }
    };
}

macro_rules! eq {
    ($e:expr, $t:expr) => {
        unsafe { $e as *mut PyObject == $t as *mut PyObject }
    };
}

pub fn to_schema(p: ObjectRef) -> PyResult<Schema> {
    if p.is(ptr!(PyBool_Type)) {
        Ok(Schema::Primitive(Primitive::Bool))
    } else if p.is(ptr!(PyUnicode_Type)) {
        Ok(Schema::Primitive(Primitive::Str))
    } else if p.is(ptr!(PyLong_Type)) {
        Ok(Schema::Primitive(Primitive::Int))
    } else if p.is(ptr!(PyFloat_Type)) {
        Ok(Schema::Primitive(Primitive::Float))
    } else if p.is(ptr!(PyBytes_Type)) {
        Ok(Schema::Primitive(Primitive::Bytes))
    } else if p.is(ptr!(PyByteArray_Type)) {
        Ok(Schema::Primitive(Primitive::ByteArray))
    } else if let Some(s) = maybe_dataclass(p)? {
        Ok(s)
    } else if let Some(s) = maybe_generic(p)? {
        Ok(s)
    } else if let Some(s) = maybe_enum(p)? {
        Ok(s)
    } else {
        Err(pyerr("unsupported type"))
    }
}

fn maybe_dataclass(p: ObjectRef) -> PyResult<Option<Schema>> {
    if !p.has_attr("__dataclass_fields__\0") {
        return Ok(None);
    }

    let arg = types::Tuple::new1(p)?;
    let fields = static_objects()?.fields.call(arg)?;
    let fields = types::Tuple::from(fields);

    let mut members = new_indexmap();

    for i in 0..fields.len() {
        let field = fields.getref(i)?;
        let name = field.get_attr("name\0")?;
        let ty = field.get_attr("type\0")?;

        let s = name.as_str()?;
        let mem = FieldSchema::new(
            s.into(),
            i as usize,
            FieldAttr::default(),
            to_schema(ty.as_ref())?,
        );
        members.insert(s.to_string(), mem);
    }

    let name = p.typename();
    let class = types::Class::new(p.to_owned());

    Ok(Some(Schema::Class(Class::new(
        class,
        name.into(),
        ClassAttr::default(),
        members,
        new_indexmap(),
    ))))
}

fn maybe_enum(p: ObjectRef) -> PyResult<Option<Schema>> {
    if !p.is_typeof(static_objects()?.enum_meta.as_ptr()) {
        return Ok(None);
    }

    let iter = p.get_iter()?;

    let variants: PyResult<_> = iter
        .map(|item| {
            let name = item.get_attr("name\0")?;
            let value = item.get_attr("value\0")?;

            let name = name.as_str()?;

            Ok((
                name.to_string(),
                VariantSchema::new(name.into(), VariantAttr::default(), value),
            ))
        })
        .collect();

    Ok(Some(Schema::Enum(Enum::new(
        EnumAttr::default(),
        variants?,
    ))))
}

fn maybe_option(args: TupleRef) -> PyResult<Schema> {
    let t1 = args.get(0)?;
    let t2 = args.get(1)?;
    let s = if t1.is(obj_none_type()) {
        let t = to_schema(t2)?;
        Schema::Optional(Optional::new(Box::new(t)))
    } else if t2.is(obj_none_type()) {
        let t = to_schema(t1)?;
        Schema::Optional(Optional::new(Box::new(t)))
    } else {
        Schema::Union(Union::new(vec![to_schema(t1)?, to_schema(t2)?]))
    };
    Ok(s)
}

fn to_union(p: ObjectRef) -> PyResult<Schema> {
    let args = get_args(p)?;
    let args = args.as_ref();
    let iter = args.iter();

    if iter.len() == 2 {
        return maybe_option(args);
    }

    let variants: PyResult<Vec<_>> = iter.map(|arg| to_schema(arg)).collect();
    Ok(Schema::Union(Union::new(variants?)))
}

fn to_tuple(p: ObjectRef) -> PyResult<Schema> {
    let args = get_args(p)?;
    let args = args.as_ref();
    let iter = args.iter();

    let args: PyResult<_> = iter.map(|arg| to_schema(arg)).collect();
    Ok(Schema::Tuple(Tuple::new(args?)))
}

fn to_dict(p: ObjectRef) -> PyResult<Schema> {
    let args = get_args(p)?;
    let args = args.as_ref();
    let key = to_schema(args.get(0)?)?;
    let value = to_schema(args.get(1)?)?;
    Ok(Schema::Dict(Dict::new(Box::new(key), Box::new(value))))
}

fn to_list(p: ObjectRef) -> PyResult<Schema> {
    let args = get_args(p)?;
    let args = args.as_ref();
    let value = to_schema(args.get(0)?)?;
    Ok(Schema::List(List::new(Box::new(value))))
}

fn to_set(p: ObjectRef) -> PyResult<Schema> {
    let args = get_args(p)?;
    let args = args.as_ref();
    let value = to_schema(args.get(1)?)?;
    Ok(Schema::Set(Set::new(Box::new(value))))
}

fn get_args(p: ObjectRef) -> PyResult<types::Tuple> {
    Ok(types::Tuple::from(p.get_attr("__args__\0")?))
}

fn maybe_generic(p: ObjectRef) -> PyResult<Option<Schema>> {
    if !p.is_typeof(static_objects()?.generic_alias.as_ptr()) {
        return Ok(None);
    }

    let origin = p.get_attr("__origin__\0")?;

    let s = if origin.is(static_objects()?.union.as_ptr()) {
        to_union(p)?
    } else if origin.is(static_objects()?.tuple.as_ptr()) {
        to_tuple(p)?
    } else if origin.is(ptr!(PyDict_Type)) {
        to_dict(p)?
    } else if origin.is(ptr!(PySet_Type)) {
        to_set(p)?
    } else if origin.is(ptr!(PyList_Type)) {
        to_list(p)?
    } else if origin.is(ptr!(PyFrozenSet_Type)) {
        unimplemented!()
    } else {
        return Ok(None);
    };

    Ok(Some(s))
}

pub struct StaticObjects {
    fields: StaticObject,
    generic_alias: StaticObject,
    union: StaticObject,
    tuple: StaticObject,
    enum_meta: StaticObject,
}

fn static_objects() -> PyResult<&'static StaticObjects> {
    STATIC_OBJECTS.as_ref().map_err(|e| pyerr(e))
}

macro_rules! getattr {
    ($module:expr, $name:expr) => {
        $module
            .getattr($name)
            .map(|p| pyo3::PyObject::from(p).into())
            .map_err(|_| concat!("couldn't find function `", $name, "`"))
    };
}

lazy_static::lazy_static! {
    static ref STATIC_OBJECTS: Result<StaticObjects, &'static str> = {
        let dataclasses = PyModule::import(py(), "dataclasses")
            .map_err(|_| "couldn't import `dataclasses`")?;
        let typing = PyModule::import(py(), "typing")
            .map_err(|_| "couldn't import `typing`")?;
        let enum_ = PyModule::import(py(), "enum")
            .map_err(|_| "couldn't import `enum`")?;

        let fields = getattr!(dataclasses, "fields")?;
        let generic_alias = getattr!(typing, "_GenericAlias")?;
        let union = getattr!(typing, "Union")?;
        let tuple = getattr!(typing, "Tuple")?;
        let enum_meta = getattr!(enum_, "EnumMeta")?;

        Ok(StaticObjects {
            fields,
            generic_alias,
            union,
            tuple,
            enum_meta,
        })
    };
}
