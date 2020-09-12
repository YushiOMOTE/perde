use super::{Object, ObjectRef};
use pyo3::{conversion::AsPyPointer, ffi::*, PyResult};

#[derive(Debug, Clone)]
pub struct TupleRef(ObjectRef);

impl TupleRef {
    pub fn new(args: ObjectRef) -> Self {
        Self(args)
    }

    pub fn len(&self) -> usize {
        unsafe { PyTuple_Size(self.0.as_ptr()) as usize }
    }

    pub fn get(&self, index: usize) -> PyResult<ObjectRef> {
        objref!(PyTuple_GET_ITEM(self.0.as_ptr(), index as Py_ssize_t))
    }

    pub fn iter(&self) -> TupleRefIter {
        TupleRefIter {
            p: self.clone(),
            len: self.len(),
            index: 0,
        }
    }
}

#[derive(Debug)]
pub struct TupleRefIter {
    p: TupleRef,
    len: usize,
    index: usize,
}

impl TupleRefIter {
    pub fn len(&self) -> usize {
        self.len
    }
}

impl Iterator for TupleRefIter {
    type Item = ObjectRef;

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
    pub fn new(len: usize) -> PyResult<Self> {
        Ok(Self(objnew!(PyTuple_New(len as Py_ssize_t))?))
    }

    pub fn new1(a1: ObjectRef) -> PyResult<Self> {
        let mut t = Self::new(1)?;
        t.setref(0, a1);
        Ok(t)
    }

    pub fn new2(a1: ObjectRef, a2: ObjectRef) -> PyResult<Self> {
        let mut t = Self::new(2)?;
        t.setref(0, a1);
        t.setref(1, a2);
        Ok(t)
    }

    pub fn set(&mut self, index: usize, obj: Object) {
        unsafe {
            // This API steals the pointer, so use `into_ptr`.
            PyTuple_SET_ITEM(self.0.as_ptr(), index as Py_ssize_t, obj.into_ptr());
        }
    }

    pub fn setref(&mut self, index: usize, objref: ObjectRef) {
        unsafe {
            PyTuple_SetItem(self.0.as_ptr(), index as Py_ssize_t, objref.as_ptr());
        }
    }

    pub fn getref(&self, index: usize) -> PyResult<ObjectRef> {
        objref!(PyTuple_GetItem(self.0.as_ptr(), index as Py_ssize_t))
    }

    pub fn len(&self) -> usize {
        unsafe { PyTuple_Size(self.0.as_ptr()) as usize }
    }

    pub fn as_ptr(&self) -> *mut PyObject {
        self.0.as_ptr()
    }

    pub fn as_ref(&self) -> TupleRef {
        TupleRef(ObjectRef::new(self.0.as_ptr()).unwrap())
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
