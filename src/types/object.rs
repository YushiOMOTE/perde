use super::Tuple;
use crate::util::*;
use pyo3::{
    conversion::{AsPyPointer, IntoPyPointer},
    ffi::*,
    PyErr, PyResult,
};
use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    os::raw::c_char,
    ptr::NonNull,
};

macro_rules! objnew {
    ($p:expr) => {
        $crate::types::Object::new(unsafe { $p })
    };
}

macro_rules! objclone {
    ($p:expr) => {
        $crate::types::Object::new_clone(unsafe { $p })
    };
}

macro_rules! is_type {
    ($p:expr, $t:expr) => {
        unsafe { $p == &mut $t as *mut _ as *mut PyObject }
    };
}

pub fn obj_none() -> PyResult<Object> {
    objclone!(Py_None())
}

pub fn obj_true() -> PyResult<Object> {
    objclone!(Py_True())
}

pub fn obj_false() -> PyResult<Object> {
    objclone!(Py_False())
}

pub fn obj_bool(b: bool) -> PyResult<Object> {
    match b {
        true => obj_true(),
        false => obj_false(),
    }
}

pub fn obj_i64(value: i64) -> PyResult<Object> {
    objnew!(PyLong_FromLongLong(value))
}

pub fn obj_u64(value: u64) -> PyResult<Object> {
    objnew!(PyLong_FromUnsignedLongLong(value))
}

pub fn obj_f64(value: f64) -> PyResult<Object> {
    objnew!(PyFloat_FromDouble(value))
}

pub fn obj_str(value: &str) -> PyResult<Object> {
    objnew!(PyUnicode_FromStringAndSize(
        value.as_ptr() as *const c_char,
        value.len() as Py_ssize_t
    ))
}

pub fn obj_bytes(value: &[u8]) -> PyResult<Object> {
    objnew!(PyBytes_FromStringAndSize(
        value.as_ptr() as *const c_char,
        value.len() as Py_ssize_t
    ))
}

pub fn obj_bytearray(value: &[u8]) -> PyResult<Object> {
    objnew!(PyByteArray_FromStringAndSize(
        value.as_ptr() as *const c_char,
        value.len() as Py_ssize_t
    ))
}

#[derive(Debug, Clone, Copy)]
pub struct ObjectPtr(NonNull<PyObject>);

impl ObjectPtr {
    pub fn new(p: *mut PyObject) -> PyResult<Self> {
        match NonNull::new(p) {
            Some(p) => Ok(Self(p)),
            None => Err(PyErr::fetch(py())),
        }
    }

    pub fn is(&self, p: *mut PyObject) -> bool {
        self.0.as_ptr() == p
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

    pub fn as_str(&self) -> PyResult<&str> {
        let mut len: Py_ssize_t = 0;
        let mut p = unsafe { PyUnicode_AsUTF8AndSize(self.as_ptr(), &mut len) };

        if p.is_null() {
            Err(pyerr("object is not a string"))
        } else {
            unsafe {
                let slice = std::slice::from_raw_parts(p as *const u8, len as usize);
                Ok(std::str::from_utf8(slice).unwrap())
            }
        }
    }

    pub fn as_ptr(&self) -> *mut PyObject {
        self.0.as_ptr()
    }

    pub fn has_attr(&self, s: &str) -> bool {
        unsafe { PyObject_HasAttrString(self.0.as_ptr(), s.as_ptr() as *mut c_char) != 0 }
    }

    pub fn get_attr(&self, s: &str) -> PyResult<Object> {
        objnew!(PyObject_GetAttrString(
            self.0.as_ptr(),
            s.as_ptr() as *mut c_char
        ))
    }

    pub fn get_iter(&self) -> PyResult<ObjectIter> {
        Ok(ObjectIter(objnew!(PyObject_GetIter(self.as_ptr()))?))
    }

    pub fn is_none_type(&self) -> bool {
        is_type!(self.0.as_ptr(), (*Py_None()).ob_type)
    }

    pub fn is_bool(&self) -> bool {
        is_type!(self.0.as_ptr(), PyBool_Type)
    }

    pub fn is_str(&self) -> bool {
        is_type!(self.0.as_ptr(), PyUnicode_Type)
    }

    pub fn is_int(&self) -> bool {
        is_type!(self.0.as_ptr(), PyLong_Type)
    }

    pub fn is_float(&self) -> bool {
        is_type!(self.0.as_ptr(), PyFloat_Type)
    }

    pub fn is_bytes(&self) -> bool {
        is_type!(self.0.as_ptr(), PyBytes_Type)
    }

    pub fn is_bytearray(&self) -> bool {
        is_type!(self.0.as_ptr(), PyByteArray_Type)
    }

    pub fn is_dict(&self) -> bool {
        is_type!(self.0.as_ptr(), PyDict_Type)
    }

    pub fn is_set(&self) -> bool {
        is_type!(self.0.as_ptr(), PySet_Type)
    }

    pub fn is_list(&self) -> bool {
        is_type!(self.0.as_ptr(), PyList_Type)
    }

    pub fn is_fronzen_set(&self) -> bool {
        is_type!(self.0.as_ptr(), PyFrozenSet_Type)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ObjectRef<'a>(ObjectPtr, PhantomData<&'a ()>);

impl<'a> ObjectRef<'a> {
    pub unsafe fn new(p: *mut PyObject) -> PyResult<Self> {
        Ok(Self(ObjectPtr::new(p)?, PhantomData))
    }

    pub fn to_owned(&self) -> Object {
        Object::new_clone(self.as_ptr()).unwrap()
    }

    pub fn store_item<T>(&self, s: &str, item: T) -> PyResult<&'a T> {
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

        if unsafe {
            PyObject_SetAttrString(self.0.as_ptr(), s.as_ptr() as *mut c_char, obj.as_ptr()) != 0
        } {
            Err(PyErr::fetch(py()))
        } else {
            Ok(p)
        }
    }

    pub fn load_item<T>(&self, s: &str) -> PyResult<&'a T> {
        let obj = self.get_attr(s)?;

        let p = unsafe { PyCapsule_GetPointer(obj.as_ptr(), std::ptr::null_mut()) };

        if p.is_null() {
            Err(PyErr::fetch(py()))
        } else {
            Ok(unsafe { &*(p as *mut T) })
        }
    }
}

impl<'a> Deref for ObjectRef<'a> {
    type Target = ObjectPtr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for ObjectRef<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

#[derive(Debug)]
pub struct Object(ObjectPtr);

impl Object {
    pub fn new(p: *mut PyObject) -> PyResult<Self> {
        Ok(Self(ObjectPtr::new(p)?))
    }

    pub fn new_clone(p: *mut PyObject) -> PyResult<Self> {
        let o = Self::new(p)?;
        o.incref();
        Ok(o)
    }

    pub fn into_ptr(self) -> *mut PyObject {
        let ptr = self.0.as_ptr();
        std::mem::forget(self);
        ptr
    }

    pub fn call(&self, tuple: Tuple) -> PyResult<Object> {
        objnew!(PyObject_CallObject(self.0.as_ptr(), tuple.as_ptr()))
    }

    pub fn as_ref<'a>(&'a self) -> ObjectRef<'a> {
        unsafe { ObjectRef::<'a>::new(self.0.as_ptr()).unwrap() }
    }

    fn incref(&self) {
        unsafe { Py_INCREF(self.0.as_ptr()) }
    }

    fn decref(&self) {
        unsafe { Py_DECREF(self.0.as_ptr()) }
    }
}

impl Deref for Object {
    type Target = ObjectPtr;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Clone for Object {
    fn clone(&self) -> Self {
        unsafe {
            Py_INCREF(self.0.as_ptr());
        }
        Self(self.0)
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        self.decref()
    }
}

#[derive(Debug)]
pub struct StaticObject(Object);

impl StaticObject {
    fn as_ptr(&self) -> *mut PyObject {
        self.0.as_ptr()
    }
}

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
    pub generic_alias: StaticObject,
    pub union: StaticObject,
    pub tuple: StaticObject,
    pub enum_meta: StaticObject,
}

unsafe impl Sync for StaticObject {}

pub fn static_objects() -> PyResult<&'static StaticObjects> {
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
        use pyo3::types::PyModule;

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
