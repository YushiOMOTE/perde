use super::{AttrStr, Tuple};
use crate::{
    error::Result,
    resolve::resolve_schema,
    schema::{Primitive, Schema, WithSchema},
};
use pyo3::{conversion::IntoPyPointer, ffi::*};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    os::raw::c_char,
    ptr::NonNull,
};

macro_rules! objnew {
    ($p:expr) => {
        $crate::types::Object::new(unsafe { $p })
    };
}

macro_rules! cast {
    ($p:expr) => {
        unsafe { &mut $p as *mut _ as *mut PyObject }
    };
}

macro_rules! is_typing {
    ($object:expr, $name:ident) => {
        static_objects()
            .ok()
            .filter(|o| $object.is(o.$name.as_ptr()))
            .is_some()
    };
}

#[derive(Debug)]
pub struct ObjectRef;

impl ObjectRef {
    pub fn new<'a>(p: *mut PyObject) -> Result<&'a Self> {
        match unsafe { (p as *mut ObjectRef).as_ref() } {
            Some(p) => Ok(p),
            None => Err(err!("failed to create an object")),
        }
    }

    pub fn resolve<'a>(&'a self, attr: Option<HashMap<&str, &ObjectRef>>) -> Result<&'a Schema> {
        resolve_schema(self, attr)
    }

    pub fn resolved_object<'a>(&'a self) -> Result<WithSchema<'a>> {
        let schema = self.get_type()?.resolve(None)?;
        Ok(WithSchema::new(schema, self))
    }

    pub fn with_schema<'a>(&'a self, schema: &'a Schema) -> WithSchema<'a> {
        WithSchema::new(schema, self)
    }

    pub fn owned(&self) -> Object {
        Object::new_clone(self.as_ptr()).unwrap()
    }

    pub fn get_type(&self) -> Result<&ObjectRef> {
        Self::new(unsafe { (*self.as_ptr()).ob_type } as *mut PyObject)
    }

    pub fn set_capsule<'a, T>(&self, s: &AttrStr, item: T) -> Result<&'a T> {
        extern "C" fn destructor(p: *mut PyObject) {
            let p = unsafe { PyCapsule_GetPointer(p, std::ptr::null_mut()) };
            let _b = unsafe { Box::from_raw(p) };
        }

        let p = Box::new(item);
        let p = Box::leak(p);

        let obj = Object::new(unsafe {
            PyCapsule_New(
                p as *mut _ as *mut std::ffi::c_void,
                std::ptr::null_mut(),
                Some(destructor),
            )
        })?;

        if unsafe { PyObject_SetAttrString(self.as_ptr(), s.as_ptr(), obj.as_ptr()) != 0 } {
            bail!("cannot set attribute `{}`", s)
        } else {
            Ok(p)
        }
    }

    pub fn get_capsule<'a, T>(&self, s: &AttrStr) -> Option<&'a T> {
        if !self.has_attr(s) {
            return None;
        }
        let obj = self.get_attr(s).ok()?;

        let p = unsafe { PyCapsule_GetPointer(obj.as_ptr(), std::ptr::null_mut()) };

        if p.is_null() {
            None
        } else {
            Some(unsafe { &*(p as *mut T) })
        }
    }

    pub fn as_bool(&self) -> Result<bool> {
        if self.is(unsafe { Py_True() }) {
            Ok(true)
        } else if self.is(unsafe { Py_False() }) {
            Ok(false)
        } else {
            bail!("object is not boolean type")
        }
    }

    pub fn as_i64(&self) -> Result<i64> {
        let p = unsafe { PyLong_AsLongLong(self.as_ptr()) };
        if unsafe { !PyErr_Occurred().is_null() } {
            bail!("object is not integer type")
        } else {
            Ok(p)
        }
    }

    pub fn as_u64(&self) -> Result<u64> {
        let p = unsafe { PyLong_AsLongLong(self.as_ptr()) };
        if unsafe { !PyErr_Occurred().is_null() } {
            bail!("object is not integer type")
        } else {
            Ok(p as u64)
        }
    }

    pub fn as_f64(&self) -> Result<f64> {
        let p = unsafe { PyFloat_AsDouble(self.as_ptr()) };
        if unsafe { !PyErr_Occurred().is_null() } {
            bail!("object is not double float")
        } else {
            Ok(p)
        }
    }

    pub fn as_str<'a>(&'a self) -> Result<&'a str> {
        let mut len: Py_ssize_t = 0;
        let p = unsafe { PyUnicode_AsUTF8AndSize(self.as_ptr(), &mut len) };

        if p.is_null() {
            bail!("object is not a string")
        } else {
            unsafe {
                let slice = std::slice::from_raw_parts(p as *const u8, len as usize);
                Ok(std::str::from_utf8(slice).unwrap())
            }
        }
    }

    pub fn as_bytes<'a>(&'a self) -> Result<&'a [u8]> {
        let mut len: Py_ssize_t = 0;
        let mut buf: *mut c_char = std::ptr::null_mut();
        let p = unsafe { PyBytes_AsStringAndSize(self.as_ptr(), &mut buf, &mut len) };

        if p == -1 {
            bail!("object is not bytes")
        } else {
            unsafe {
                let slice = std::slice::from_raw_parts(buf as *const u8, len as usize);
                Ok(slice)
            }
        }
    }

    pub fn as_bytearray<'a>(&'a self) -> Result<&'a [u8]> {
        let p = unsafe { PyByteArray_AsString(self.as_ptr()) };
        let len = unsafe { PyByteArray_Size(self.as_ptr()) };

        if p.is_null() {
            bail!("object is not bytearray")
        } else {
            unsafe {
                let slice = std::slice::from_raw_parts(p as *const u8, len as usize);
                Ok(slice)
            }
        }
    }

    pub fn is(&self, p: *mut PyObject) -> bool {
        self.as_ptr() == p
    }

    pub fn is_none(&self) -> bool {
        self.is(unsafe { Py_None() })
    }

    pub fn is_none_type(&self) -> bool {
        self.is(unsafe { (*Py_None()).ob_type as *mut PyObject })
    }

    pub fn is_bool(&self) -> bool {
        self.is(cast!(PyBool_Type))
    }

    pub fn is_str(&self) -> bool {
        self.is(cast!(PyUnicode_Type))
    }

    pub fn is_int(&self) -> bool {
        self.is(cast!(PyLong_Type))
    }

    pub fn is_float(&self) -> bool {
        self.is(cast!(PyFloat_Type))
    }

    pub fn is_bytes(&self) -> bool {
        self.is(cast!(PyBytes_Type))
    }

    pub fn is_bytearray(&self) -> bool {
        self.is(cast!(PyByteArray_Type))
    }

    pub fn is_dict(&self) -> bool {
        self.is(cast!(PyDict_Type)) || is_typing!(self, dict)
    }

    pub fn is_tuple(&self) -> bool {
        self.is(cast!(PyTuple_Type)) || is_typing!(self, tuple)
    }

    pub fn is_set(&self) -> bool {
        self.is(cast!(PySet_Type)) || is_typing!(self, set)
    }

    pub fn is_list(&self) -> bool {
        self.is(cast!(PyList_Type)) || is_typing!(self, list)
    }

    pub fn is_frozen_set(&self) -> bool {
        self.is(cast!(PyFrozenSet_Type)) || is_typing!(self, frozenset)
    }

    pub fn is_any(&self) -> bool {
        // `Any`, bare `Optional` and bare `Union` can be treated as Any.
        is_typing!(self, any) || is_typing!(self, optional) || is_typing!(self, union)
    }

    pub fn is_instance(&self, p: *mut PyObject) -> bool {
        unsafe { (*self.as_ptr()).ob_type as *mut PyObject == p }
    }

    pub fn name(&self) -> &str {
        unsafe {
            std::ffi::CStr::from_ptr((*(self.as_ptr() as *mut PyTypeObject)).tp_name)
                .to_str()
                .unwrap_or("<unknown>")
        }
    }

    pub fn as_ptr(&self) -> *mut PyObject {
        &*self as *const Self as *mut Self as *mut PyObject
    }

    pub fn has_attr(&self, s: &AttrStr) -> bool {
        unsafe { PyObject_HasAttrString(self.as_ptr(), s.as_ptr()) != 0 }
    }

    pub fn get_attr(&self, s: &AttrStr) -> Result<Object> {
        objnew!(PyObject_GetAttrString(self.as_ptr(), s.as_ptr()))
    }

    pub fn get_iter(&self) -> Result<ObjectIter> {
        Ok(ObjectIter(objnew!(PyObject_GetIter(self.as_ptr()))?))
    }

    pub fn get(&self, s: &str) -> Option<Object> {
        let key = Object::new_str(s).ok()?;
        objnew!(PyObject_GetItem(self.as_ptr(), key.as_ptr())).ok()
    }

    pub fn call(&self, tuple: Tuple) -> Result<Object> {
        objnew!(PyObject_CallObject(self.as_ptr(), tuple.as_ptr()))
    }

    pub fn call_noarg(&self) -> Result<Object> {
        self.call(Tuple::new(0)?)
    }
}

#[derive(Debug)]
pub struct ObjectIter(Object);

impl Iterator for ObjectIter {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        let p = unsafe { PyIter_Next(self.0.as_ptr()) };
        if p.is_null() {
            None
        } else {
            Some(Object::new(p).unwrap())
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Object(NonNull<ObjectRef>);

impl Object {
    pub fn new(p: *mut PyObject) -> Result<Self> {
        match NonNull::new(p as *mut ObjectRef) {
            Some(p) => Ok(Self(p)),
            None => Err(err!("failed to create an object")),
        }
    }

    pub fn new_clone(p: *mut PyObject) -> Result<Self> {
        let o = Self::new(p)?;
        o.incref();
        Ok(o)
    }

    pub fn new_none() -> Object {
        Self::new_clone(unsafe { Py_None() }).unwrap()
    }

    pub fn new_bool(b: bool) -> Object {
        let ptr = match b {
            true => unsafe { Py_True() },
            false => unsafe { Py_False() },
        };
        Self::new_clone(ptr).unwrap()
    }

    pub fn new_i64(v: i64) -> Result<Object> {
        Self::new(unsafe { PyLong_FromLongLong(v) })
    }

    pub fn new_u64(v: u64) -> Result<Object> {
        Self::new(unsafe { PyLong_FromUnsignedLongLong(v) })
    }

    pub fn new_f64(v: f64) -> Result<Object> {
        Self::new(unsafe { PyFloat_FromDouble(v) })
    }

    pub fn new_str(v: &str) -> Result<Object> {
        Self::new(unsafe {
            PyUnicode_FromStringAndSize(v.as_ptr() as *const c_char, v.len() as Py_ssize_t)
        })
    }

    pub fn new_bytes(v: &[u8]) -> Result<Object> {
        Self::new(unsafe {
            PyBytes_FromStringAndSize(v.as_ptr() as *const c_char, v.len() as Py_ssize_t)
        })
    }

    pub fn new_bytearray(v: &[u8]) -> Result<Object> {
        Self::new(unsafe {
            PyByteArray_FromStringAndSize(v.as_ptr() as *const c_char, v.len() as Py_ssize_t)
        })
    }

    pub fn new_default(s: &Schema) -> Result<Object> {
        let obj = match s {
            Schema::Primitive(Primitive::Bool) => {
                ObjectRef::new(cast!(PyBool_Type))?.call_noarg()?
            }
            Schema::Primitive(Primitive::Int) => {
                ObjectRef::new(cast!(PyLong_Type))?.call_noarg()?
            }
            Schema::Primitive(Primitive::Float) => {
                ObjectRef::new(cast!(PyFloat_Type))?.call_noarg()?
            }
            Schema::Primitive(Primitive::Str) => {
                ObjectRef::new(cast!(PyUnicode_Type))?.call_noarg()?
            }
            Schema::Primitive(Primitive::Bytes) => {
                ObjectRef::new(cast!(PyBytes_Type))?.call_noarg()?
            }
            Schema::Primitive(Primitive::ByteArray) => {
                ObjectRef::new(cast!(PyByteArray_Type))?.call_noarg()?
            }
            Schema::Primitive(Primitive::DateTime) => static_objects()?.datetime.call_noarg()?,
            Schema::Primitive(Primitive::Date) => static_objects()?.date.call_noarg()?,
            Schema::Primitive(Primitive::Time) => static_objects()?.time.call_noarg()?,
            Schema::Primitive(Primitive::Decimal) => static_objects()?.decimal.call_noarg()?,
            Schema::Primitive(Primitive::Uuid) => static_objects()?.uuid.call_noarg()?,
            Schema::Dict(_) => ObjectRef::new(cast!(PyDict_Type))?.call_noarg()?,
            Schema::List(_) => ObjectRef::new(cast!(PyList_Type))?.call_noarg()?,
            Schema::Set(_) => ObjectRef::new(cast!(PySet_Type))?.call_noarg()?,
            Schema::FrozenSet(_) => ObjectRef::new(cast!(PyFrozenSet_Type))?.call_noarg()?,
            Schema::Tuple(_) => bail!("cannot use default construction for `tuple`"),
            Schema::Class(c) => c.ty.default_construct()?,
            Schema::Enum(_) => bail!("cannot use default construction for `enum`"),
            Schema::Union(_) => bail!("cannot use default construction for `union`"),
            Schema::Any(_) => bail!("cannot use default construction for `any`"),
        };
        Ok(obj)
    }

    pub fn into_ptr(self) -> *mut PyObject {
        let ptr = self.0.as_ptr();
        std::mem::forget(self);
        ptr as *mut PyObject
    }

    pub fn none_as_optional(self) -> Option<Object> {
        if self.is_none() {
            None
        } else {
            Some(self)
        }
    }

    fn incref(&self) {
        unsafe { Py_INCREF(self.as_ptr()) }
    }

    fn decref(&self) {
        unsafe { Py_DECREF(self.as_ptr()) }
    }
}

impl Deref for Object {
    type Target = ObjectRef;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.as_mut() }
    }
}

impl AsRef<ObjectRef> for Object {
    fn as_ref(&self) -> &ObjectRef {
        &self
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        self.deref().owned()
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        self.decref()
    }
}

#[derive(Debug)]
pub struct StaticObject(Object);

impl Deref for StaticObject {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<pyo3::PyObject> for StaticObject {
    fn from(p: pyo3::PyObject) -> Self {
        StaticObject(Object::new(p.into_ptr()).unwrap())
    }
}

pub struct StaticObjects {
    pub fields: StaticObject,
    pub missing: StaticObject,
    pub generic_alias: StaticObject,
    pub base_generic_alias: Option<StaticObject>,
    pub union_generic_alias: Option<StaticObject>,
    pub special_generic_alias: Option<StaticObject>,
    pub type_var: StaticObject,
    pub any: StaticObject,
    pub union: StaticObject,
    pub tuple: StaticObject,
    pub empty_tuple: StaticObject,
    pub optional: StaticObject,
    pub dict: StaticObject,
    pub list: StaticObject,
    pub set: StaticObject,
    pub frozenset: StaticObject,
    pub enum_meta: StaticObject,
    pub datetime: StaticObject,
    pub date: StaticObject,
    pub time: StaticObject,
    pub decimal: StaticObject,
    pub uuid: StaticObject,
}

pub fn is_datetime(obj: &ObjectRef) -> Result<bool> {
    Ok(obj.is(static_objects()?.datetime.as_ptr()))
}

pub fn is_date(obj: &ObjectRef) -> Result<bool> {
    Ok(obj.is(static_objects()?.date.as_ptr()))
}

pub fn is_time(obj: &ObjectRef) -> Result<bool> {
    Ok(obj.is(static_objects()?.time.as_ptr()))
}

pub fn is_decimal(obj: &ObjectRef) -> Result<bool> {
    Ok(obj.is(static_objects()?.decimal.as_ptr()))
}

pub fn is_uuid(obj: &ObjectRef) -> Result<bool> {
    Ok(obj.is(static_objects()?.uuid.as_ptr()))
}

pub fn isoformat(obj: &ObjectRef) -> Result<Object> {
    obj.get_attr(&ATTR_ISOFORMAT)?.call_noarg()
}

pub fn datetime_fromisoformat(obj: &ObjectRef) -> Result<Object> {
    let mut args = Tuple::new(1)?;
    args.set(0, obj.owned());
    static_objects()?
        .datetime
        .get_attr(&ATTR_FROMISOFORMAT)?
        .call(args)
}

pub fn date_fromisoformat(obj: &ObjectRef) -> Result<Object> {
    let mut args = Tuple::new(1)?;
    args.set(0, obj.owned());
    static_objects()?
        .date
        .get_attr(&ATTR_FROMISOFORMAT)?
        .call(args)
}

pub fn time_fromisoformat(obj: &ObjectRef) -> Result<Object> {
    let mut args = Tuple::new(1)?;
    args.set(0, obj.owned());
    static_objects()?
        .time
        .get_attr(&ATTR_FROMISOFORMAT)?
        .call(args)
}

pub fn to_str(obj: &ObjectRef) -> Result<Object> {
    let strtype = ObjectRef::new(cast!(PyUnicode_Type))?;
    let mut args = Tuple::new(1)?;
    args.set(0, obj.owned());
    strtype.call(args)
}

pub fn to_uuid(obj: &ObjectRef) -> Result<Object> {
    let mut args = Tuple::new(1)?;
    args.set(0, obj.owned());
    static_objects()?.uuid.call(args)
}

pub fn to_decimal(obj: &ObjectRef) -> Result<Object> {
    let mut args = Tuple::new(1)?;
    args.set(0, obj.owned());
    static_objects()?.decimal.call(args)
}

unsafe impl Sync for StaticObject {}

pub fn static_objects() -> Result<&'static StaticObjects> {
    STATIC_OBJECTS.as_ref().map_err(|e| err!("{}", e))
}

macro_rules! getattr {
    ($module:expr, $name:expr) => {
        $module
            .getattr($name)
            .map(|p| pyo3::PyObject::from(p).into())
            .map_err(|_| err!(concat!("couldn't find function `", $name, "`")))
    };
}

lazy_static::lazy_static! {
    static ref ATTR_ISOFORMAT: AttrStr = AttrStr::new("isoformat");
    static ref ATTR_FROMISOFORMAT: AttrStr = AttrStr::new("fromisoformat");

    static ref STATIC_OBJECTS: Result<StaticObjects> = {
        use pyo3::{Python, types::PyModule};

        let py = unsafe { Python::assume_gil_acquired() };

        let dataclasses = PyModule::import(py, "dataclasses")
            .map_err(|_| err!("couldn't import `dataclasses`"))?;
        let typing = PyModule::import(py, "typing")
            .map_err(|_| err!("couldn't import `typing`"))?;
        let enum_ = PyModule::import(py, "enum")
            .map_err(|_| err!("couldn't import `enum`"))?;
        let datetime_ = PyModule::import(py, "datetime")
            .map_err(|_| err!("couldn't import `datetime`"))?;
        let decimal_ = PyModule::import(py, "decimal")
            .map_err(|_| err!("couldn't import `decimal`"))?;
        let uuid_ = PyModule::import(py, "uuid")
            .map_err(|_| err!("couldn't import `uuid`"))?;

        let fields = getattr!(dataclasses, "fields")?;
        let missing = getattr!(dataclasses, "MISSING")?;
        let generic_alias = getattr!(typing, "_GenericAlias")?;
        let union_generic_alias = getattr!(typing, "_UnionGenericAlias").ok();
        let base_generic_alias = getattr!(typing, "_BaseGenericAlias").ok();
        let special_generic_alias = getattr!(typing, "_SpecialGenericAlias").ok();
        let type_var = getattr!(typing, "TypeVar")?;
        let any = getattr!(typing, "Any")?;
        let union = getattr!(typing, "Union")?;
        let tuple = getattr!(typing, "Tuple")?;
        let optional = getattr!(typing, "Optional")?;
        let dict = getattr!(typing, "Dict")?;
        let list = getattr!(typing, "List")?;
        let set = getattr!(typing, "Set")?;
        let frozenset = getattr!(typing, "FrozenSet")?;
        let enum_meta = getattr!(enum_, "EnumMeta")?;

        let tuple_type = ObjectRef::new(cast!(PyTuple_Type))?;
        let empty_tuple = StaticObject(tuple_type.call_noarg()?);

        let datetime = getattr!(datetime_, "datetime")?;
        let date = getattr!(datetime_, "date")?;
        let time = getattr!(datetime_, "time")?;
        let decimal = getattr!(decimal_, "Decimal")?;
        let uuid = getattr!(uuid_, "UUID")?;

        Ok(StaticObjects {
            fields,
            missing,
            generic_alias,
            union_generic_alias,
            base_generic_alias,
            special_generic_alias,
            type_var,
            any,
            union,
            tuple,
            empty_tuple,
            optional,
            dict,
            list,
            set,
            frozenset,
            enum_meta,
            datetime,
            date,
            time,
            decimal,
            uuid,
        })
    };
}
