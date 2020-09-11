use crate::util::*;
use pyo3::ffi::*;
use pyo3::{PyErr, PyResult};
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::NonNull;

macro_rules! objnew {
    ($p:expr) => {
        $crate::types::Object::new(unsafe { $p })
    };
}

macro_rules! objref {
    ($p:expr) => {
        $crate::types::ObjectRef::new(unsafe { $p })
    };
}

macro_rules! objclone {
    ($p:expr) => {
        $crate::types::Object::new_clone(unsafe { $p })
    };
}

#[derive(Debug, Clone)]
pub struct ObjectRef(NonNull<PyObject>);

impl ObjectRef {
    pub fn new(p: *mut PyObject) -> PyResult<Self> {
        match NonNull::new(p) {
            Some(p) => Ok(Self(p)),
            None => Err(PyErr::fetch(py())),
        }
    }

    pub fn typeref(&self) -> PyResult<ObjectRef> {
        Self::new(unsafe { (*self.as_ptr()).ob_type as *mut PyObject })
    }

    pub fn is_typeof(&self, p: *mut PyObject) -> bool {
        unsafe { (*self.as_ptr()).ob_type as *mut PyObject == p }
    }

    pub fn name(&self) -> &str {
        unsafe {
            std::ffi::CStr::from_ptr((*(self.as_ptr() as *mut PyTypeObject)).tp_name)
                .to_str()
                .unwrap_or("<unknown>")
        }
    }

    pub fn typename(&self) -> &str {
        unsafe {
            std::ffi::CStr::from_ptr((*(*self.as_ptr()).ob_type).tp_name)
                .to_str()
                .unwrap_or("<unknown>")
        }
    }

    pub fn as_ptr(&self) -> *mut PyObject {
        self.0.as_ptr()
    }
}

#[derive(Debug, Clone)]
pub struct Args(ObjectRef);

impl Args {
    pub fn new(args: *mut PyObject) -> PyResult<Self> {
        Ok(Self(ObjectRef::new(args)?))
    }

    pub fn get(&self, index: usize) -> PyResult<ObjectRef> {
        objref!(PyTuple_GET_ITEM(self.0.as_ptr(), index as Py_ssize_t))
    }
}

#[derive(Debug)]
pub struct Object(NonNull<PyObject>);

impl Clone for Object {
    fn clone(&self) -> Self {
        unsafe {
            Py_INCREF(self.0.as_ptr());
        }
        Self(self.0)
    }
}

impl Object {
    pub fn new(p: *mut PyObject) -> PyResult<Self> {
        match NonNull::new(p) {
            Some(p) => Ok(Self(p)),
            None => Err(PyErr::fetch(py())),
        }
    }

    pub fn new_clone(p: *mut PyObject) -> PyResult<Self> {
        let o = Self::new(p)?;
        o.incref();
        Ok(o)
    }

    pub fn as_ptr(&self) -> *mut PyObject {
        self.0.as_ptr()
    }

    pub fn into_ptr(self) -> *mut PyObject {
        let ptr = self.0.as_ptr();
        std::mem::forget(self);
        ptr
    }

    pub fn typeref(&self) -> PyResult<ObjectRef> {
        ObjectRef::new(unsafe { (*self.as_ptr()).ob_type as *mut PyObject })
    }

    pub fn is_typeof(&self, p: *mut PyObject) -> bool {
        unsafe { (*self.as_ptr()).ob_type as *mut PyObject == p }
    }

    pub fn name(&self) -> &str {
        unsafe {
            std::ffi::CStr::from_ptr((*(self.as_ptr() as *mut PyTypeObject)).tp_name)
                .to_str()
                .unwrap_or("<unknown>")
        }
    }

    pub fn typename(&self) -> &str {
        unsafe {
            std::ffi::CStr::from_ptr((*(*self.as_ptr()).ob_type).tp_name)
                .to_str()
                .unwrap_or("<unknown>")
        }
    }

    fn incref(&self) {
        unsafe { Py_INCREF(self.0.as_ptr()) }
    }

    fn decref(&self) {
        unsafe { Py_DECREF(self.0.as_ptr()) }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        self.decref()
    }
}

pub fn as_str(p: &Object) -> PyResult<&str> {
    let mut len: Py_ssize_t = 0;
    let mut p = unsafe { PyUnicode_AsUTF8AndSize(p.as_ptr(), &mut len) };
    if p.is_null() {
        Err(pyerr("object is not a string"))
    } else {
        Ok(unsafe { std::ffi::CStr::from_ptr(p).to_str().unwrap() })
    }
}

pub fn py_none() -> PyResult<Object> {
    objclone!(Py_None())
}

pub fn py_true() -> PyResult<Object> {
    objclone!(Py_True())
}

pub fn py_false() -> PyResult<Object> {
    objclone!(Py_False())
}

pub fn py_bool(b: bool) -> PyResult<Object> {
    match b {
        true => py_true(),
        false => py_false(),
    }
}

pub fn py_i64(value: i64) -> PyResult<Object> {
    objnew!(PyLong_FromLongLong(value))
}

pub fn py_u64(value: u64) -> PyResult<Object> {
    objnew!(PyLong_FromUnsignedLongLong(value))
}

pub fn py_f64(value: f64) -> PyResult<Object> {
    objnew!(PyFloat_FromDouble(value))
}

pub fn py_str(value: &str) -> PyResult<Object> {
    objnew!(crate::unicode::unicode_from_str(value))
    // objnew!(PyUnicode_FromStringAndSize(
    //     value.as_ptr() as *const c_char,
    //     value.len() as Py_ssize_t
    // ))
}

pub fn py_bytes(value: &[u8]) -> PyResult<Object> {
    objnew!(PyBytes_FromStringAndSize(
        value.as_ptr() as *const c_char,
        value.len() as Py_ssize_t
    ))
}

pub fn py_bytearray(value: &[u8]) -> PyResult<Object> {
    objnew!(PyByteArray_FromStringAndSize(
        value.as_ptr() as *const c_char,
        value.len() as Py_ssize_t
    ))
}

#[derive(Debug, Clone)]
pub struct List(Object);

impl List {
    pub fn new(len: usize) -> PyResult<Self> {
        Ok(Self(objnew!(PyList_New(len as Py_ssize_t))?))
    }

    pub fn set(&mut self, index: usize, obj: Object) {
        unsafe {
            // This API steals the pointer, so use `into_ptr`.
            PyList_SET_ITEM(self.0.as_ptr(), index as Py_ssize_t, obj.into_ptr());
        }
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Set(Object);

impl Set {
    pub fn new() -> PyResult<Self> {
        Ok(Self(objnew!(PySet_New(std::ptr::null_mut()))?))
    }

    pub fn set(&mut self, obj: Object) -> PyResult<()> {
        unsafe {
            // This API doesn't steal.
            if PySet_Add(self.0.as_ptr(), obj.as_ptr()) == -1 {
                return Err(PyErr::fetch(py()));
            }
        }
        Ok(())
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Dict(Object);

impl Dict {
    pub fn new() -> PyResult<Self> {
        Ok(Self(objnew!(PyDict_New())?))
    }

    pub fn set(&mut self, key: Object, value: Object) -> PyResult<()> {
        unsafe {
            // This API doesn't steal.
            if PyDict_SetItem(self.0.as_ptr(), key.as_ptr(), value.as_ptr()) == 1 {
                return Err(PyErr::fetch(py()));
            }
        }
        Ok(())
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Tuple(Object);

impl Tuple {
    pub fn new(len: usize) -> PyResult<Self> {
        Ok(Self(objnew!(PyTuple_New(len as Py_ssize_t))?))
    }

    pub fn set(&mut self, index: usize, obj: Object) {
        unsafe {
            // This API steals the pointer, so use `into_ptr`.
            PyTuple_SET_ITEM(self.0.as_ptr(), index as Py_ssize_t, obj.into_ptr());
        }
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Class(Object);

impl Class {
    pub fn new(obj: Object) -> Self {
        Self(obj)
    }

    pub fn construct(&self, args: Tuple) -> PyResult<Object> {
        let p = args.into_inner();
        // This API doesn't steal.
        let r = objnew!(PyObject_Call(
            self.0.as_ptr(),
            p.as_ptr(),
            std::ptr::null_mut()
        ))?;
        Ok(r)
    }

    pub fn is_typeof(&self, p: *mut PyObject) -> bool {
        p == self.0.as_ptr()
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Enum(Object);

impl Enum {
    pub fn new(obj: Object) -> Self {
        Self(obj)
    }

    pub fn value(&self, name: &str) -> PyResult<Object> {
        let s = CString::new(name).map_err(pyerr)?;
        // This API does return a new reference.
        objnew!(PyObject_GetAttrString(self.0.as_ptr(), s.as_ptr()))
    }

    pub fn is_typeof(&self, p: *mut PyObject) -> bool {
        p == self.0.as_ptr()
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}
