use super::{Object, ObjectRef};
use crate::error::Result;
use pyo3::ffi::*;

#[derive(Debug, Clone)]
pub struct TupleRef<'a>(&'a ObjectRef);

impl<'a> TupleRef<'a> {
    pub fn new(args: &'a ObjectRef) -> Self {
        Self(args)
    }

    pub fn from_args(args: *mut PyObject) -> Result<Self> {
        Ok(Self(ObjectRef::new(args)?))
    }

    pub fn len(&self) -> usize {
        unsafe { PyTuple_Size(self.0.as_ptr()) as usize }
    }

    pub fn get(&self, index: usize) -> Result<&'a ObjectRef> {
        unsafe { ObjectRef::new(PyTuple_GET_ITEM(self.0.as_ptr(), index as Py_ssize_t)) }
    }

    pub fn iter(&self) -> TupleRefIter<'a> {
        TupleRefIter {
            p: self.clone(),
            len: self.len(),
            index: 0,
        }
    }
}

#[derive(Debug)]
pub struct TupleRefIter<'a> {
    p: TupleRef<'a>,
    len: usize,
    index: usize,
}

impl<'a> TupleRefIter<'a> {
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<'a> Iterator for TupleRefIter<'a> {
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
pub struct Tuple(Object);

impl Tuple {
    pub fn new(len: usize) -> Result<Self> {
        Ok(Self(objnew!(PyTuple_New(len as Py_ssize_t))?))
    }

    pub fn one(a1: &ObjectRef) -> Result<Self> {
        let mut t = Self::new(1)?;
        t.setref(0, a1);
        Ok(t)
    }

    pub fn set(&mut self, index: usize, obj: Object) {
        unsafe {
            // This API steals the pointer, so use `into_ptr`.
            PyTuple_SET_ITEM(self.0.as_ptr(), index as Py_ssize_t, obj.into_ptr());
        }
    }

    pub fn setref(&mut self, index: usize, objref: &ObjectRef) {
        unsafe {
            PyTuple_SetItem(self.0.as_ptr(), index as Py_ssize_t, objref.as_ptr());
        }
    }

    pub fn getref<'a>(&'a self, index: usize) -> Result<&'a ObjectRef> {
        unsafe { ObjectRef::new(PyTuple_GetItem(self.0.as_ptr(), index as Py_ssize_t)) }
    }

    pub fn len(&self) -> usize {
        unsafe { PyTuple_Size(self.0.as_ptr()) as usize }
    }

    pub fn as_ptr(&self) -> *mut PyObject {
        self.0.as_ptr()
    }

    pub fn as_ref(&self) -> TupleRef {
        TupleRef::new(&self.0)
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

impl From<Object> for Tuple {
    fn from(obj: Object) -> Self {
        Self(obj)
    }
}
