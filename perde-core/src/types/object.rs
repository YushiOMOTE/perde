use super::{AttrStr, Tuple};
use crate::{
    error::Result,
    import::import,
    resolve::resolve_schema,
    schema::{Schema, WithSchema},
};
use pyo3::ffi::*;
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

macro_rules! ptr_cast {
    ($p:expr) => {
        unsafe { $p as *mut _ as *mut PyObject }
    };
}

macro_rules! is_type {
    ($object:expr, $name:ident) => {
        import()
            .ok()
            .filter(|o| $object.is(o.$name.as_ptr()))
            .is_some()
    };
}

macro_rules! is_type_opt {
    ($object:expr, $name:ident) => {
        import()
            .ok()
            .and_then(|o| o.$name.as_ref())
            .filter(|o| $object.is(o.as_ptr()))
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
        Self::new(ptr_cast!((*self.as_ptr()).ob_type))
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

    pub fn as_list<'a>(&'a self) -> ListRef<'a> {
        ListRef::new(self)
    }

    pub fn as_set<'a>(&'a self) -> SetRef<'a> {
        SetRef::new(self)
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
        self.is(cast!(PyDict_Type)) || is_type!(self, dict)
    }

    pub fn is_tuple(&self) -> bool {
        self.is(cast!(PyTuple_Type)) || is_type!(self, tuple)
    }

    pub fn is_set(&self) -> bool {
        self.is(cast!(PySet_Type)) || is_type!(self, set)
    }

    pub fn is_list(&self) -> bool {
        self.is(cast!(PyList_Type)) || is_type!(self, list)
    }

    pub fn is_frozen_set(&self) -> bool {
        self.is(cast!(PyFrozenSet_Type)) || is_type!(self, frozenset)
    }

    pub fn is_any(&self) -> bool {
        // `Any`, bare `Optional` and bare `Union` can be treated as Any.
        is_type!(self, any) || is_type!(self, optional) || is_type!(self, union)
    }

    pub fn is_generic(&self) -> bool {
        self.get_type()
            .ok()
            .filter(|o| {
                is_type!(o, generic_alias)
                    || is_type_opt!(o, base_generic_alias)
                    || is_type_opt!(o, union_generic_alias)
                    || is_type_opt!(o, special_generic_alias)
            })
            .is_some()
    }

    pub fn is_enum(&self) -> bool {
        self.get_type()
            .ok()
            .filter(|o| is_type!(o, enum_meta))
            .is_some()
    }

    pub fn is_datetime(&self) -> bool {
        is_type!(self, datetime)
    }

    pub fn is_date(&self) -> bool {
        is_type!(self, date)
    }

    pub fn is_time(&self) -> bool {
        is_type!(self, time)
    }

    pub fn is_decimal(&self) -> bool {
        is_type!(self, decimal)
    }

    pub fn is_uuid(&self) -> bool {
        is_type!(self, uuid)
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

    pub fn get_tuple_iter<'a>(&'a self) -> Result<TupleIter<'a>> {
        TupleIter::new(self)
    }

    pub fn get_dict_iter<'a>(&'a self) -> Result<DictIter<'a>> {
        DictIter::new(self)
    }

    pub fn get(&self, s: &str) -> Result<Object> {
        let key = Object::new_str(s)?;
        objnew!(PyObject_GetItem(self.as_ptr(), key.as_ptr()))
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
    type Item = Result<Object>;

    fn next(&mut self) -> Option<Self::Item> {
        let p = unsafe { PyIter_Next(self.0.as_ptr()) };
        if p.is_null() {
            if unsafe { !PyErr_Occurred().is_null() } {
                Some(Err(err!("an error occurred during iteration")))
            } else {
                None
            }
        } else {
            Some(Ok(Object::new(p).unwrap()))
        }
    }
}

#[derive(Debug)]
pub struct TupleIter<'a> {
    p: &'a ObjectRef,
    len: usize,
    index: usize,
}

impl<'a> TupleIter<'a> {
    fn new(p: &'a ObjectRef) -> Result<Self> {
        let len = unsafe { PyTuple_Size(p.as_ptr()) as usize };
        if unsafe { !PyErr_Occurred().is_null() } {
            bail!("cannot get the size of tuple")
        }
        Ok(Self { p, len, index: 0 })
    }

    fn get(&self, index: usize) -> Result<&'a ObjectRef> {
        unsafe { ObjectRef::new(PyTuple_GET_ITEM(self.p.as_ptr(), index as Py_ssize_t)) }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<'a> Iterator for TupleIter<'a> {
    type Item = &'a ObjectRef;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            None
        } else {
            let item = self.get(self.index).ok();
            self.index += 1;
            item
        }
    }
}

#[derive(Debug, Clone)]
pub struct DictIter<'a> {
    p: &'a ObjectRef,
    len: usize,
    index: Py_ssize_t,
}

impl<'a> DictIter<'a> {
    fn new(p: &'a ObjectRef) -> Result<Self> {
        let len = unsafe { PyDict_Size(p.as_ptr()) as usize };
        if unsafe { !PyErr_Occurred().is_null() } {
            bail!("cannot get the size of dict")
        }
        Ok(Self { p, len, index: 0 })
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<'a> Iterator for DictIter<'a> {
    type Item = (&'a ObjectRef, &'a ObjectRef);

    fn next(&mut self) -> Option<Self::Item> {
        let mut k = std::ptr::null_mut();
        let mut v = std::ptr::null_mut();

        let res = unsafe { PyDict_Next(self.p.as_ptr(), &mut self.index, &mut k, &mut v) };

        if res == 0 {
            None
        } else {
            let k = ObjectRef::new(k).ok()?;
            let v = ObjectRef::new(v).ok()?;
            Some((k, v))
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
            Schema::Bool => ObjectRef::new(cast!(PyBool_Type))?.call_noarg()?,
            Schema::Int => ObjectRef::new(cast!(PyLong_Type))?.call_noarg()?,
            Schema::Float => ObjectRef::new(cast!(PyFloat_Type))?.call_noarg()?,
            Schema::Str => ObjectRef::new(cast!(PyUnicode_Type))?.call_noarg()?,
            Schema::Bytes => ObjectRef::new(cast!(PyBytes_Type))?.call_noarg()?,
            Schema::ByteArray => ObjectRef::new(cast!(PyByteArray_Type))?.call_noarg()?,
            Schema::DateTime => import()?.datetime.call_noarg()?,
            Schema::Date => import()?.date.call_noarg()?,
            Schema::Time => import()?.time.call_noarg()?,
            Schema::Decimal => import()?.decimal.call_noarg()?,
            Schema::Uuid => import()?.uuid.call_noarg()?,
            Schema::Dict(_) => ObjectRef::new(cast!(PyDict_Type))?.call_noarg()?,
            Schema::List(_) => ObjectRef::new(cast!(PyList_Type))?.call_noarg()?,
            Schema::Set(_) => ObjectRef::new(cast!(PySet_Type))?.call_noarg()?,
            Schema::FrozenSet(_) => ObjectRef::new(cast!(PyFrozenSet_Type))?.call_noarg()?,
            Schema::Tuple(_) => bail!("cannot use default construction for `tuple`"),
            Schema::Class(c) => c.ty.call_noarg()?,
            Schema::Enum(_) => bail!("cannot use default construction for `enum`"),
            Schema::Union(_) => bail!("cannot use default construction for `union`"),
            Schema::Any(_) => bail!("cannot use default construction for `any`"),
        };
        Ok(obj)
    }

    pub fn build_list(len: usize) -> Result<ListBuilder> {
        ListBuilder::new(len)
    }

    pub fn build_set() -> Result<SetBuilder> {
        SetBuilder::new()
    }

    pub fn build_dict() -> Result<DictBuilder> {
        DictBuilder::new()
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

#[derive(Debug, Clone)]
pub struct SetRef<'a>(&'a ObjectRef);

impl<'a> SetRef<'a> {
    fn new(obj: &'a ObjectRef) -> Self {
        Self(obj)
    }

    pub fn len(&self) -> usize {
        unsafe { PySet_Size(self.0.as_ptr()) as usize }
    }
}

#[derive(Debug, Clone)]
pub struct ListRef<'a>(&'a ObjectRef);

impl<'a> ListRef<'a> {
    fn new(obj: &'a ObjectRef) -> Self {
        Self(obj)
    }

    pub fn len(&self) -> usize {
        unsafe { PyList_Size(self.0.as_ptr()) as usize }
    }

    pub fn get(&self, index: usize) -> Option<&'a ObjectRef> {
        let p = unsafe { PyList_GetItem(self.0.as_ptr(), index as Py_ssize_t) };
        if p.is_null() {
            None
        } else {
            Some(ObjectRef::new(p).ok()?)
        }
    }
}

lazy_static::lazy_static! {
    static ref ATTR_ISOFORMAT: AttrStr = AttrStr::new("isoformat");
    static ref ATTR_FROMISOFORMAT: AttrStr = AttrStr::new("fromisoformat");
}

pub fn isoformat(obj: &ObjectRef) -> Result<Object> {
    obj.get_attr(&ATTR_ISOFORMAT)?.call_noarg()
}

pub fn datetime_fromisoformat(obj: &ObjectRef) -> Result<Object> {
    let mut args = Tuple::new(1)?;
    args.set(0, obj.owned());
    import()?.datetime.get_attr(&ATTR_FROMISOFORMAT)?.call(args)
}

pub fn date_fromisoformat(obj: &ObjectRef) -> Result<Object> {
    let mut args = Tuple::new(1)?;
    args.set(0, obj.owned());
    import()?.date.get_attr(&ATTR_FROMISOFORMAT)?.call(args)
}

pub fn time_fromisoformat(obj: &ObjectRef) -> Result<Object> {
    let mut args = Tuple::new(1)?;
    args.set(0, obj.owned());
    import()?.time.get_attr(&ATTR_FROMISOFORMAT)?.call(args)
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
    import()?.uuid.call(args)
}

pub fn to_decimal(obj: &ObjectRef) -> Result<Object> {
    let mut args = Tuple::new(1)?;
    args.set(0, obj.owned());
    import()?.decimal.call(args)
}

#[derive(Debug, Clone)]
pub struct ListBuilder(Object);

impl ListBuilder {
    fn new(len: usize) -> Result<Self> {
        Ok(Self(objnew!(PyList_New(len as Py_ssize_t))?))
    }

    pub fn set(&mut self, index: usize, obj: Object) {
        unsafe {
            // This API steals the pointer, so use `into_ptr`.
            PyList_SET_ITEM(self.0.as_ptr(), index as Py_ssize_t, obj.into_ptr());
        }
    }

    pub fn build(self) -> Object {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct SetBuilder(Object);

impl SetBuilder {
    fn new() -> Result<Self> {
        Ok(Self(objnew!(PySet_New(std::ptr::null_mut()))?))
    }

    pub fn set(&mut self, obj: Object) -> Result<()> {
        unsafe {
            // This API doesn't steal.
            if PySet_Add(self.0.as_ptr(), obj.as_ptr()) != 0 {
                bail!("cannot add an item to a set")
            }
        }
        Ok(())
    }

    pub fn build_frozen(self) -> Result<Object> {
        objnew!(PyFrozenSet_New(self.0.as_ptr()))
    }

    pub fn build(self) -> Object {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct DictBuilder(Object);

impl DictBuilder {
    fn new() -> Result<Self> {
        Ok(Self(objnew!(PyDict_New())?))
    }

    pub fn set(&mut self, key: Object, value: Object) -> Result<()> {
        unsafe {
            // This API doesn't steal.
            if PyDict_SetItem(self.0.as_ptr(), key.as_ptr(), value.as_ptr()) != 0 {
                bail!("cannot set an item to dictionary")
            }
        }
        Ok(())
    }

    pub fn build(self) -> Object {
        self.0
    }
}
