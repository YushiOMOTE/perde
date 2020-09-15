use super::{Object, ObjectRef, Tuple};
use crate::util::*;
use pyo3::{conversion::AsPyPointer, ffi::*, PyErr, PyResult};
use std::os::raw::c_char;

#[derive(Debug, Clone)]
pub struct ListRef<'a>(ObjectRef<'a>);

impl<'a> ListRef<'a> {
    pub fn new(obj: ObjectRef<'a>) -> Self {
        Self(obj)
    }

    pub fn len(&self) -> usize {
        unsafe { PyList_Size(self.0.as_ptr()) as usize }
    }

    pub fn get(&self, index: usize) -> Option<ObjectRef> {
        let p = unsafe { PyList_GetItem(self.0.as_ptr(), index as Py_ssize_t) };
        if p.is_null() {
            None
        } else {
            Some(unsafe { ObjectRef::new(p).ok()? })
        }
    }
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
pub struct SetRef<'a>(ObjectRef<'a>);

impl<'a> SetRef<'a> {
    pub fn new(obj: ObjectRef<'a>) -> Self {
        Self(obj)
    }

    pub fn len(&self) -> usize {
        unsafe { PySet_Size(self.0.as_ptr()) as usize }
    }

    pub fn get(&self, index: usize) -> Option<ObjectRef> {
        unimplemented!()
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
            if PySet_Add(self.0.as_ptr(), obj.as_ptr()) != 0 {
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
pub struct DictRef<'a>(ObjectRef<'a>);

impl<'a> DictRef<'a> {
    pub fn new(obj: ObjectRef<'a>) -> Self {
        Self(obj)
    }

    pub fn len(&self) -> usize {
        unsafe { PyDict_Size(self.0.as_ptr()) as usize }
    }

    pub fn iter(&self) -> DictRefIter<'a> {
        DictRefIter {
            obj: self.0,
            index: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DictRefIter<'a> {
    obj: ObjectRef<'a>,
    index: Py_ssize_t,
}

impl<'a> Iterator for DictRefIter<'a> {
    type Item = (ObjectRef<'a>, ObjectRef<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut k = std::ptr::null_mut();
        let mut v = std::ptr::null_mut();

        let res = unsafe { PyDict_Next(self.obj.as_ptr(), &mut self.index, &mut k, &mut v) };

        if res == 0 {
            None
        } else {
            let k = unsafe { ObjectRef::new(k).ok()? };
            let v = unsafe { ObjectRef::new(v).ok()? };
            Some((k, v))
        }
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
            if PyDict_SetItem(self.0.as_ptr(), key.as_ptr(), value.as_ptr()) != 0 {
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
pub struct ClassRef<'a>(ObjectRef<'a>);

impl<'a> ClassRef<'a> {
    pub fn new(obj: ObjectRef<'a>) -> Self {
        Self(obj)
    }

    pub fn get(&self, name: &str) -> PyResult<Object> {
        Object::new(unsafe {
            PyObject_GetAttrString(self.0.as_ptr(), name.as_ptr() as *const c_char)
        })
    }
}

#[derive(Debug, Clone)]
pub struct Class(Object);

impl Class {
    pub fn new(obj: Object) -> Self {
        Self(obj)
    }

    pub fn construct(&self, args: Tuple) -> PyResult<Object> {
        self.0.call(args)
    }

    pub fn name(&self) -> &str {
        self.0.name()
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
        self.0.get_attr(name)
    }

    pub fn is_typeof(&self, p: *mut PyObject) -> bool {
        p == self.0.as_ptr()
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}
