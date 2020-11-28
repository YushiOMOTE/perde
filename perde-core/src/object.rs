use crate::{
    attr::AttrStr,
    error::Result,
    import::import,
    resolve::resolve_schema,
    schema::{Schema, WithSchema},
};
use pyo3::ffi::*;
use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::{self, Debug},
    ops::{Deref, DerefMut},
    os::raw::c_char,
    ptr::NonNull,
    sync::atomic::{AtomicPtr, Ordering},
};

macro_rules! objnew {
    ($p:expr) => {
        $crate::object::Object::new(unsafe { $p })
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

pub struct ObjectRef;

impl ObjectRef {
    pub fn new<'a>(p: *mut PyObject) -> Result<&'a Self> {
        match unsafe { (p as *mut ObjectRef).as_ref() } {
            Some(p) => Ok(p),
            None => Err(err!("failed to create an object")),
        }
    }

    pub fn resolve<'a>(
        &'a self,
        attr: Option<HashMap<&str, &ObjectRef>>,
    ) -> Result<Cow<'a, Schema>> {
        resolve_schema(self, attr)
    }

    pub fn resolved_object(&self) -> Result<WithSchema<'_>> {
        let schema = self.get_type()?.resolve(None)?;
        Ok(WithSchema::new(schema, self))
    }

    pub fn with_schema<'a>(&'a self, schema: &'a Schema) -> WithSchema<'a> {
        WithSchema::new(schema.borrowed(), self)
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
            bail_type_err!("expected `bool` got `{}`: {:?}", self.typename(), self)
        }
    }

    pub fn as_i64(&self) -> Result<i64> {
        let p = unsafe { PyLong_AsLongLong(self.as_ptr()) };
        if unsafe { !PyErr_Occurred().is_null() } {
            bail_type_err!("expected `int` got `{}`: {:?}", self.typename(), self)
        } else {
            Ok(p)
        }
    }

    pub fn as_u64(&self) -> Result<u64> {
        let p = unsafe { PyLong_AsLongLong(self.as_ptr()) };
        if unsafe { !PyErr_Occurred().is_null() } {
            bail_type_err!("expected `int` got `{}`: {:?}", self.typename(), self)
        } else {
            Ok(p as u64)
        }
    }

    pub fn as_f64(&self) -> Result<f64> {
        let p = unsafe { PyFloat_AsDouble(self.as_ptr()) };
        if unsafe { !PyErr_Occurred().is_null() } {
            bail_type_err!("expected `float` got `{}`: {:?}", self.typename(), self)
        } else {
            Ok(p)
        }
    }

    pub fn as_str(&self) -> Result<&str> {
        let mut len: Py_ssize_t = 0;
        let p = unsafe { PyUnicode_AsUTF8AndSize(self.as_ptr(), &mut len) };

        if p.is_null() {
            bail_type_err!("expected `str` got `{}`: {:?}", self.typename(), self)
        } else {
            unsafe {
                let slice = std::slice::from_raw_parts(p as *const u8, len as usize);
                Ok(std::str::from_utf8(slice).unwrap())
            }
        }
    }

    pub fn as_bytes(&self) -> Result<&[u8]> {
        let mut len: Py_ssize_t = 0;
        let mut buf: *mut c_char = std::ptr::null_mut();
        let p = unsafe { PyBytes_AsStringAndSize(self.as_ptr(), &mut buf, &mut len) };

        if p == -1 {
            bail_type_err!("expected `bytes` got `{}`: {:?}", self.typename(), self)
        } else {
            unsafe {
                let slice = std::slice::from_raw_parts(buf as *const u8, len as usize);
                Ok(slice)
            }
        }
    }

    pub fn as_bytearray(&self) -> Result<&[u8]> {
        let p = unsafe { PyByteArray_AsString(self.as_ptr()) };
        let len = unsafe { PyByteArray_Size(self.as_ptr()) };

        if p.is_null() {
            bail_type_err!("expected `bytearray` got `{}`: {:?}", self.typename(), self)
        } else {
            unsafe {
                let slice = std::slice::from_raw_parts(p as *const u8, len as usize);
                Ok(slice)
            }
        }
    }

    pub fn as_list(&self) -> ListRef<'_> {
        ListRef::new(self)
    }

    pub fn as_set(&self) -> SetRef<'_> {
        SetRef::new(self)
    }

    pub fn as_tuple(&self) -> TupleRef<'_> {
        TupleRef::new(self)
    }

    pub fn to_str(&self) -> Result<Object> {
        Object::new(unsafe { PyObject_Str(self.as_ptr()) })
    }

    pub fn is(&self, p: *mut PyObject) -> bool {
        self.as_ptr() == p
    }

    pub fn is_none(&self) -> bool {
        self.is(unsafe { Py_None() })
    }

    pub fn is_type(&self) -> bool {
        unsafe { (*self.as_ptr()).ob_type == &mut PyType_Type }
    }

    pub fn is_none_type(&self) -> bool {
        self.is(ptr_cast!((*Py_None()).ob_type))
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

    pub fn is_builtin_generic(&self) -> bool {
        self.get_type()
            .ok()
            .filter(|o| is_type_opt!(o, types_generic_alias))
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
            if self.is_type() {
                let p = (*(self.as_ptr() as *mut PyTypeObject)).tp_name;
                std::ffi::CStr::from_ptr(p)
                    .to_str()
                    .unwrap_or("__unknown__")
            } else {
                "__unknown__"
            }
        }
    }

    pub fn typename(&self) -> &str {
        self.get_type().map(|t| t.name()).unwrap_or("__unknown__")
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

    pub fn get_tuple_iter(&self) -> Result<TupleIter<'_>> {
        TupleIter::new(self)
    }

    pub fn get_dict_iter(&self) -> Result<DictIter<'_>> {
        DictIter::new(self)
    }

    pub fn get(&self, s: &str) -> Result<Object> {
        let key = Object::new_str(s)?;
        objnew!(PyObject_GetItem(self.as_ptr(), key.as_ptr()))
    }

    pub fn call(&self, args: Vec<Object>) -> Result<Object> {
        let mut tuple = Object::build_tuple(args.len())?;
        for (i, arg) in args.into_iter().enumerate() {
            tuple.set(i, arg);
        }
        objnew!(PyObject_CallObject(self.as_ptr(), tuple.build().as_ptr()))
    }

    pub fn call1(&self, obj: Object) -> Result<Object> {
        self.call(vec![obj])
    }

    pub fn call0(&self) -> Result<Object> {
        self.call(vec![])
    }

    pub fn isoformat(&self) -> Result<Object> {
        self.get_attr(&ATTR_ISOFORMAT)?.call0()
    }
}

impl Debug for ObjectRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.to_str()
            .ok()
            .and_then(|o| o.as_str().ok().map(|s| write!(f, "{}", s)))
            .unwrap_or_else(|| write!(f, "<unknown>"))
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
    p: TupleRef<'a>,
    len: usize,
    index: usize,
}

impl<'a> TupleIter<'a> {
    fn new(p: &'a ObjectRef) -> Result<Self> {
        let len = unsafe { PyTuple_Size(p.as_ptr()) as usize };
        if unsafe { !PyErr_Occurred().is_null() } {
            bail!("cannot get the size of tuple")
        }
        Ok(Self {
            p: TupleRef::new(p),
            len,
            index: 0,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
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
            let item = self.p.get(self.index).ok();
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

    pub fn is_empty(&self) -> bool {
        self.len == 0
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

#[derive(PartialEq, Eq)]
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

    pub fn new_unit() -> Result<Object> {
        let tuple_type = ObjectRef::new(cast!(PyTuple_Type))?;
        tuple_type.call0()
    }

    pub fn new_default(s: &Schema) -> Result<Object> {
        let obj = match s {
            Schema::Bool => ObjectRef::new(cast!(PyBool_Type))?.call0()?,
            Schema::Int => ObjectRef::new(cast!(PyLong_Type))?.call0()?,
            Schema::Float => ObjectRef::new(cast!(PyFloat_Type))?.call0()?,
            Schema::Str => ObjectRef::new(cast!(PyUnicode_Type))?.call0()?,
            Schema::Bytes => ObjectRef::new(cast!(PyBytes_Type))?.call0()?,
            Schema::ByteArray => ObjectRef::new(cast!(PyByteArray_Type))?.call0()?,
            Schema::DateTime => import()?.datetime.call0()?,
            Schema::Date => import()?.date.call0()?,
            Schema::Time => import()?.time.call0()?,
            Schema::Decimal => import()?.decimal.call0()?,
            Schema::Uuid => import()?.uuid.call0()?,
            Schema::Dict(_) => ObjectRef::new(cast!(PyDict_Type))?.call0()?,
            Schema::List(_) => ObjectRef::new(cast!(PyList_Type))?.call0()?,
            Schema::Set(_) => ObjectRef::new(cast!(PySet_Type))?.call0()?,
            Schema::FrozenSet(_) => ObjectRef::new(cast!(PyFrozenSet_Type))?.call0()?,
            Schema::Tuple(_) => bail!("cannot use default construction for `tuple`"),
            Schema::Class(c) => c.ty.call0()?,
            Schema::Enum(_) => bail!("cannot use default construction for `enum`"),
            Schema::Union(_) => bail!("cannot use default construction for `union`"),
            Schema::Any(_) => bail!("cannot use default construction for `any`"),
        };
        Ok(obj)
    }

    pub fn into_datetime(self) -> Result<Object> {
        import()?
            .datetime
            .get_attr(&ATTR_FROMISOFORMAT)?
            .call1(self)
    }

    pub fn into_date(self) -> Result<Object> {
        import()?.date.get_attr(&ATTR_FROMISOFORMAT)?.call1(self)
    }

    pub fn into_time(self) -> Result<Object> {
        import()?.time.get_attr(&ATTR_FROMISOFORMAT)?.call1(self)
    }

    pub fn into_uuid(self) -> Result<Object> {
        import()?.uuid.call1(self)
    }

    pub fn into_decimal(self) -> Result<Object> {
        import()?.decimal.call1(self)
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

    pub fn build_tuple(len: usize) -> Result<TupleBuilder> {
        TupleBuilder::new(len)
    }

    pub fn into_ptr(self) -> *mut PyObject {
        let ptr = self.0.as_ptr();
        std::mem::forget(self);
        ptr as *mut PyObject
    }

    pub fn into_opt(self) -> Option<Object> {
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

impl Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(f)
    }
}

#[derive(Debug, Clone)]
pub struct SetRef<'a>(&'a ObjectRef);

impl<'a> SetRef<'a> {
    fn new(obj: &'a ObjectRef) -> Self {
        Self(obj)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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

#[derive(Debug, Clone)]
pub struct TupleRef<'a>(&'a ObjectRef);

impl<'a> TupleRef<'a> {
    fn new(args: &'a ObjectRef) -> Self {
        Self(args)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        unsafe { PyTuple_Size(self.0.as_ptr()) as usize }
    }

    pub fn get(&self, index: usize) -> Result<&'a ObjectRef> {
        unsafe { ObjectRef::new(PyTuple_GET_ITEM(self.0.as_ptr(), index as Py_ssize_t)) }
    }
}

lazy_static::lazy_static! {
    static ref ATTR_ISOFORMAT: AttrStr = AttrStr::new("isoformat");
    static ref ATTR_FROMISOFORMAT: AttrStr = AttrStr::new("fromisoformat");
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

#[derive(Debug, Clone)]
pub struct TupleBuilder(Object);

impl TupleBuilder {
    fn new(len: usize) -> Result<Self> {
        Ok(Self(objnew!(PyTuple_New(len as Py_ssize_t))?))
    }

    pub fn set(&mut self, index: usize, obj: Object) {
        unsafe {
            // This API steals the pointer, so use `into_ptr`.
            PyTuple_SET_ITEM(self.0.as_ptr(), index as Py_ssize_t, obj.into_ptr());
        }
    }

    pub fn build(self) -> Object {
        self.0
    }
}

pub struct SyncObject(AtomicPtr<PyObject>);

impl SyncObject {
    pub fn new(obj: Object) -> Self {
        Self(AtomicPtr::new(obj.into_ptr()))
    }

    pub fn into_ptr(self) -> *mut PyObject {
        self.as_ptr()
    }
}

impl From<Object> for SyncObject {
    fn from(obj: Object) -> Self {
        Self::new(obj)
    }
}

impl Deref for SyncObject {
    type Target = ObjectRef;

    fn deref(&self) -> &Self::Target {
        ObjectRef::new(self.0.load(Ordering::Relaxed)).unwrap()
    }
}

impl PartialEq for SyncObject {
    fn eq(&self, other: &SyncObject) -> bool {
        self.as_ptr() == other.as_ptr()
    }
}

impl Eq for SyncObject {}

impl Clone for SyncObject {
    fn clone(&self) -> Self {
        Self::new(self.owned())
    }
}

impl Drop for SyncObject {
    fn drop(&mut self) {
        let _ = Object::new(self.as_ptr());
    }
}

impl Debug for SyncObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(f)
    }
}

#[derive(Debug)]
pub struct ErrorObject {
    ptype: SyncObject,
    pvalue: SyncObject,
    ptraceback: SyncObject,
}

impl ErrorObject {
    pub fn new() -> Option<Self> {
        if unsafe { PyErr_Occurred().is_null() } {
            return None;
        }

        unsafe {
            let mut ptype = std::ptr::null_mut();
            let mut pvalue = std::ptr::null_mut();
            let mut ptraceback = std::ptr::null_mut();

            pyo3::ffi::PyErr_Fetch(&mut ptype, &mut pvalue, &mut ptraceback);

            let ptype = Object::new(ptype);
            let pvalue = Object::new(pvalue);
            let ptraceback = Object::new(ptraceback);

            Some(ErrorObject {
                ptype: ptype.ok()?.into(),
                pvalue: pvalue.ok()?.into(),
                ptraceback: ptraceback.ok()?.into(),
            })
        }
    }

    pub fn restore(self) {
        unsafe {
            pyo3::ffi::PyErr_Restore(
                self.ptype.into_ptr(),
                self.pvalue.into_ptr(),
                self.ptraceback.into_ptr(),
            )
        }
    }

    pub fn clear() {
        if unsafe { !pyo3::ffi::PyErr_Occurred().is_null() } {
            unsafe { pyo3::ffi::PyErr_Clear() };
        }
    }
}
