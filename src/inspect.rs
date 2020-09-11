use crate::{
    schema::*,
    types::{self, as_str, Object},
    unicode::read_utf8_from_str,
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

macro_rules! st {
    ($name:ident) => {
        STATIC_OBJECTS
            .as_ref()
            .map_err(|e| pyerr(e))
            .map(|o| &o.$name)
    };
}

pub fn to_schema(p: *mut PyObject) -> PyResult<Schema> {
    if is!(p, PyBool_Type) {
        Ok(Schema::Primitive(Primitive::Bool))
    } else if is!(p, PyUnicode_Type) {
        Ok(Schema::Primitive(Primitive::Str))
    } else if is!(p, PyLong_Type) {
        Ok(Schema::Primitive(Primitive::Int))
    } else if is!(p, PyFloat_Type) {
        Ok(Schema::Primitive(Primitive::Float))
    } else if is!(p, PyBytes_Type) {
        Ok(Schema::Primitive(Primitive::Bytes))
    } else if is!(p, PyByteArray_Type) {
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

fn maybe_dataclass(p: *mut PyObject) -> PyResult<Option<Schema>> {
    if unsafe { PyObject_HasAttrString(p, "__dataclass_fields__\0".as_ptr() as *mut c_char) == 0 } {
        return Ok(None);
    }

    let arg = objnew!(PyTuple_New(1))?;
    unsafe { PyTuple_SetItem(arg.as_ptr(), 0, p) };
    let fields = objnew!(PyObject_CallObject(st!(fields)?.as_ptr(), arg.as_ptr()))?;

    let len = unsafe { PyTuple_Size(fields.as_ptr()) };

    let mut members = new_indexmap();

    for i in 0..len {
        let field = objref!(PyTuple_GetItem(fields.as_ptr(), i))?;
        let name = objnew!(PyObject_GetAttrString(
            field.as_ptr(),
            "name\0".as_ptr() as *mut c_char
        ))?;
        let ty = objnew!(PyObject_GetAttrString(
            field.as_ptr(),
            "type\0".as_ptr() as *mut c_char
        ))?;

        let s = as_str(&name)?;
        let mem = FieldSchema::new(
            s.into(),
            i as usize,
            FieldAttr::default(),
            to_schema(ty.as_ptr())?,
        );
        members.insert(s.to_string(), mem);
    }

    use std::ffi::CStr;
    let name = unsafe {
        CStr::from_ptr((*(p as *mut PyTypeObject)).tp_name)
            .to_string_lossy()
            .into_owned()
    };
    let cloned = Object::new_clone(p)?;
    let class = types::Class::new(cloned);

    Ok(Some(Schema::Class(Class::new(
        class,
        name,
        ClassAttr::default(),
        members,
        new_indexmap(),
    ))))
}

fn maybe_enum(p: *mut PyObject) -> PyResult<Option<Schema>> {
    if eq!((*p).ob_type, st!(enum_meta)?.as_ptr()) {
        return Ok(None);
    }

    let iter = objnew!(PyObject_GetIter(p))?;
    let mut variants = new_indexmap();

    while let Ok(item) = objnew!(PyIter_Next(iter.as_ptr())) {
        let name = objnew!(PyObject_GetAttrString(
            item.as_ptr(),
            "name\0".as_ptr() as *mut c_char
        ))?;
        let value = objnew!(PyObject_GetAttrString(
            item.as_ptr(),
            "value\0".as_ptr() as *mut c_char
        ))?;

        let name = as_str(&name)?;
        let schema = to_schema(unsafe { (*value.as_ptr()).ob_type as *mut PyObject })?;

        variants.insert(
            name.to_string(),
            VariantSchema::new(name.into(), VariantAttr::default(), schema, value),
        );
    }

    Ok(Some(Schema::Enum(Enum::new(EnumAttr::default(), variants))))
}

fn maybe_option(args: Object) -> PyResult<Schema> {
    let t1 = unsafe { PyTuple_GetItem(args.as_ptr(), 0) };
    let t2 = unsafe { PyTuple_GetItem(args.as_ptr(), 1) };
    let s = if unsafe { t1 == (*Py_None()).ob_type as *mut PyObject } {
        let t = to_schema(t2)?;
        Schema::Optional(Optional::new(Box::new(t)))
    } else if unsafe { t2 == (*Py_None()).ob_type as *mut PyObject } {
        let t = to_schema(t1)?;
        Schema::Optional(Optional::new(Box::new(t)))
    } else {
        Schema::Union(Union::new(vec![to_schema(t1)?, to_schema(t2)?]))
    };
    Ok(s)
}

fn to_union(p: *mut PyObject) -> PyResult<Schema> {
    let args = get_args(p)?;
    let len = unsafe { PyTuple_Size(args.as_ptr()) as usize };

    if len == 2 {
        return maybe_option(args);
    }

    let mut ss = vec![];
    for i in 0..len {
        let s = to_schema(unsafe { PyTuple_GetItem(args.as_ptr(), i as isize) })?;
        ss.push(s);
    }
    Ok(Schema::Union(Union::new(ss)))
}

fn to_tuple(p: *mut PyObject) -> PyResult<Schema> {
    let args = get_args(p)?;
    let len = unsafe { PyTuple_Size(args.as_ptr()) as usize };

    let mut ss = vec![];
    for i in 0..len {
        let s = to_schema(unsafe { PyTuple_GetItem(args.as_ptr(), i as isize) })?;
        ss.push(s);
    }
    Ok(Schema::Tuple(Tuple::new(ss)))
}

fn to_dict(p: *mut PyObject) -> PyResult<Schema> {
    let args = get_args(p)?;
    let key = to_schema(unsafe { PyTuple_GetItem(args.as_ptr(), 0) })?;
    let value = to_schema(unsafe { PyTuple_GetItem(args.as_ptr(), 1) })?;
    Ok(Schema::Dict(Dict::new(Box::new(key), Box::new(value))))
}

fn to_list(p: *mut PyObject) -> PyResult<Schema> {
    let args = get_args(p)?;
    let value = to_schema(unsafe { PyTuple_GetItem(args.as_ptr(), 0) })?;
    Ok(Schema::List(List::new(Box::new(value))))
}

fn to_set(p: *mut PyObject) -> PyResult<Schema> {
    let args = get_args(p)?;
    let value = to_schema(unsafe { PyTuple_GetItem(args.as_ptr(), 0) })?;
    Ok(Schema::Set(Set::new(Box::new(value))))
}

fn get_args(p: *mut PyObject) -> PyResult<Object> {
    Object::new(unsafe { PyObject_GetAttrString(p, "__args__\0".as_ptr() as *mut c_char) })
}

fn maybe_generic(p: *mut PyObject) -> PyResult<Option<Schema>> {
    if unsafe { (*p).ob_type as *mut PyObject != st!(generic_alias)?.as_ptr() } {
        return Ok(None);
    }

    let origin =
        Object::new(unsafe { PyObject_GetAttrString(p, "__origin__\0".as_ptr() as *mut c_char) })?;

    let s = if origin.as_ptr() == st!(union)?.as_ptr() {
        to_union(p)?
    } else if origin.as_ptr() == st!(tuple)?.as_ptr() {
        to_tuple(p)?
    } else if is!(origin.as_ptr(), PyDict_Type) {
        to_dict(p)?
    } else if is!(origin.as_ptr(), PySet_Type) {
        to_set(p)?
    } else if is!(origin.as_ptr(), PyList_Type) {
        to_list(p)?
    } else if is!(origin.as_ptr(), PyFrozenSet_Type) {
        unimplemented!()
    } else {
        return Ok(None);
    };

    Ok(Some(s))
}

pub struct StaticObjects {
    fields: pyo3::PyObject,
    generic_alias: pyo3::PyObject,
    union: pyo3::PyObject,
    tuple: pyo3::PyObject,
    enum_meta: pyo3::PyObject,
}

macro_rules! getattr {
    ($module:expr, $name:expr) => {
        $module
            .getattr($name)
            .map(|p| p.into())
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

// let len = unsafe { PyDict_GET_SIZE(fields) as usize };

// let strlen: Py_ssize_t = 0;
// let pos: Py_ssize_t = 0;
// let key: *mut PyObject = std::ptr::null_mut();
// let value: *mut PyObject = std::ptr::null_mut();

// for _ in 0..len {
//     unsafe {
//         PyDict_Next(fields, &mut pos, &mut key, &mut value);
//     }

//     let key = as_str(key)?;
//     if key.as_bytes()[0] == b'_' {
//         continue;
//     }
// }

// pub mod new_deps {
//     use crate::util::*;
//     use pyo3::{ffi::*, types::PyModule, AsPyPointer, PyResult};

//     pub struct LoadedObjects {
//         is_dataclass: pyo3::PyObject,
//         fields: pyo3::PyObject,
//         get_origin: pyo3::PyObject,
//         get_args: pyo3::PyObject,
//         is_tuple_type: pyo3::PyObject,
//         is_union_type: pyo3::PyObject,
//         is_optional_type: pyo3::PyObject,
//         enum_meta: pyo3::PyObject,
//     }

//     macro_rules! getattr {
//         ($module:expr, $name:expr) => {
//             $module
//                 .getattr($name)
//                 .map(|p| p.into())
//                 .map_err(|_| concat!("couldn't find function `", $name, "`"))
//         };
//     }

//     macro_rules! objects {
//         () => {
//             OBJECTS.as_ref().map_err(pyerr)
//         };
//     }

//     macro_rules! callfunc {
//         ($name:ident, $arg:expr) => {
//             objects!().and_then(move |o| {
//                 let tuple = PyTuple_New(1);
//                 PyTuple_SetItem(tuple, 0, $arg);
//                 let res = PyObject_Call(o.$name.as_ptr(), tuple, std::ptr::null_mut());
//                 Py_DECREF(tuple);
//                 Ok(res)
//             })
//         };
//     }

//     macro_rules! is_true {
//         ($v:expr) => {{
//             let v = $v;
//             let r = PyObject_IsTrue(v) != 0;
//             Py_DECREF(v);
//             r
//         }};
//     }

//     macro_rules! is_none {
//         ($v:expr) => {{
//             let v = $v;
//             let r = v == Py_None();
//             Py_DECREF(v);
//             r
//         }};
//     }

//     lazy_static::lazy_static! {
//         static ref OBJECTS: Result<LoadedObjects, &'static str> = {
//             let dataclasses = PyModule::import(py(), "dataclasses")
//                 .map_err(|_| "couldn't import `dataclasses`")?;
//             let typing_inspect = PyModule::import(py(), "typing_inspect")
//                 .map_err(|_| "couldn't import `typing_inspect`")?;
//             let enum_ = PyModule::import(py(), "enum")
//                 .map_err(|_| "couldn't import `enum`")?;

//             let is_dataclass = getattr!(dataclasses, "is_dataclass")?;
//             let fields = getattr!(dataclasses, "fields")?;
//             let get_origin = getattr!(typing_inspect, "get_origin")?;
//             let get_args = getattr!(typing_inspect, "get_args")?;
//             let is_tuple_type = getattr!(typing_inspect, "is_tuple_type")?;
//             let is_union_type = getattr!(typing_inspect, "is_union_type")?;
//             let is_optional_type = getattr!(typing_inspect, "is_optional_type")?;
//             let enum_meta = getattr!(enum_, "EnumMeta")?;

//             Ok(LoadedObjects {
//                 is_dataclass,
//                 fields,
//                 get_origin,
//                 get_args,
//                 is_union_type,
//                 is_tuple_type,
//                 is_optional_type,
//                 enum_meta,
//             })
//         };
//     }

//     pub fn is_dataclass(ty: *mut PyObject) -> PyResult<bool> {
//         unsafe { Ok(is_true!(callfunc!(is_dataclass, ty)?)) }
//     }

//     pub fn get_origin(ty: *mut PyObject) -> PyResult<*mut PyObject> {
//         unsafe { callfunc!(get_origin, ty) }
//     }

//     pub fn get_args(ty: *mut PyObject) -> PyResult<*mut PyObject> {
//         unsafe { callfunc!(get_args, ty) }
//     }

//     pub fn is_generic(ty: *mut PyObject) -> PyResult<bool> {
//         unsafe { Ok(!is_none!(get_origin(ty)?)) }
//     }

//     pub fn is_union_type(ty: *mut PyObject) -> PyResult<bool> {
//         unsafe { Ok(is_true!(callfunc!(is_union_type, ty)?)) }
//     }

//     pub fn is_tuple_type(ty: *mut PyObject) -> PyResult<bool> {
//         unsafe { Ok(is_true!(callfunc!(is_tuple_type, ty)?)) }
//     }

//     pub fn is_optional_type(ty: *mut PyObject) -> PyResult<bool> {
//         unsafe { Ok(is_true!(callfunc!(is_optional_type, ty)?)) }
//     }

//     pub fn is_enum(ty: *mut PyObject) -> PyResult<bool> {
//         unsafe { Ok((*ty).ob_type == objects!()?.enum_meta.as_ptr() as *mut PyTypeObject) }
//     }

//     pub fn fields(ty: *mut PyObject) -> PyResult<*mut PyObject> {
//         unsafe { callfunc!(fields, ty) }
//     }

//     // pub fn is_generic(ty: *mut PyObject) -> PyResult<bool> {
//     //     let ty = any!(ty);
//     //     Ok(!get_origin(ty)?.is_none()
//     //         || is_union_type(ty)?
//     //         || is_optional_type(ty)?
//     //         || is_tuple_type(ty)?)
//     // }

//     // pub fn is_enum(ty: *mut PyObject) -> PyResult<bool> {
//     //     let ty = any!(ty);
//     //     builtins(py())?
//     //         .call1("issubclass", (ty, enum_class(py())?))
//     //         .and_then(|v| v.extract())
//     // }

//     // pub fn fields(ty: *mut PyObject) -> PyResult<Vec<&PyAny>> {
//     //     let ty = any!(ty);
//     //     dataclasses(py())?
//     //         .call1("fields", (ty,))
//     //         .and_then(|v| v.extract())
//     // }
// }
