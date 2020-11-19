use super::{Object, ObjectRef};
use crate::error::Result;
use pyo3::ffi::*;

#[derive(Debug, Clone)]
pub struct ListRef<'a>(&'a ObjectRef);

impl<'a> ListRef<'a> {
    pub fn new(obj: &'a ObjectRef) -> Self {
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

#[derive(Debug, Clone)]
pub struct List(Object);

impl List {
    pub fn new(len: usize) -> Result<Self> {
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
pub struct SetRef<'a>(&'a ObjectRef);

impl<'a> SetRef<'a> {
    pub fn new(obj: &'a ObjectRef) -> Self {
        Self(obj)
    }

    pub fn iter(&self) -> Result<SetRefIter> {
        Ok(SetRefIter(objnew!(PyObject_GetIter(self.0.as_ptr()))?))
    }

    pub fn len(&self) -> usize {
        unsafe { PySet_Size(self.0.as_ptr()) as usize }
    }
}

#[derive(Debug, Clone)]
pub struct SetRefIter(Object);

impl Iterator for SetRefIter {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        Some(
            objnew!({
                let p = PyIter_Next(self.0.as_ptr());
                if p.is_null() {
                    return None;
                }
                p
            })
            .ok()?,
        )
    }
}

#[derive(Debug, Clone)]
pub struct Set(Object);

impl Set {
    pub fn new() -> Result<Self> {
        Ok(Self(objnew!(PySet_New(std::ptr::null_mut()))?))
    }

    pub fn from_iter(obj: &ObjectRef) -> Result<Self> {
        Ok(Self(objnew!(PySet_New(obj.as_ptr()))?))
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

    pub fn iter(&self) -> Result<SetRefIter> {
        Ok(SetRefIter(objnew!(PyObject_GetIter(self.0.as_ptr()))?))
    }

    pub fn len(&self) -> usize {
        unsafe { PySet_Size(self.0.as_ptr()) as usize }
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct FrozenSet(Object);

impl FrozenSet {
    pub fn from_iter(obj: &ObjectRef) -> Result<Self> {
        Ok(Self(objnew!(PyFrozenSet_New(obj.as_ptr()))?))
    }

    pub fn into_inner(self) -> Object {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct DictRef<'a>(&'a ObjectRef);

impl<'a> DictRef<'a> {
    pub fn new(obj: &'a ObjectRef) -> Self {
        Self(obj)
    }

    pub fn len(&self) -> usize {
        unsafe { PyDict_Size(self.0.as_ptr()) as usize }
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        let key = Object::new_str(key).ok()?;
        if unsafe { PyDict_Contains(self.0.as_ptr(), key.as_ptr()) } != -1 {
            Object::new(unsafe { PyDict_GetItem(self.0.as_ptr(), key.as_ptr()) }).ok()
        } else {
            None
        }
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
    obj: &'a ObjectRef,
    index: Py_ssize_t,
}

impl<'a> Iterator for DictRefIter<'a> {
    type Item = (&'a ObjectRef, &'a ObjectRef);

    fn next(&mut self) -> Option<Self::Item> {
        let mut k = std::ptr::null_mut();
        let mut v = std::ptr::null_mut();

        let res = unsafe { PyDict_Next(self.obj.as_ptr(), &mut self.index, &mut k, &mut v) };

        if res == 0 {
            None
        } else {
            let k = ObjectRef::new(k).ok()?;
            let v = ObjectRef::new(v).ok()?;
            Some((k, v))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dict(Object);

impl Dict {
    pub fn new() -> Result<Self> {
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

    pub fn into_inner(self) -> Object {
        self.0
    }
}
