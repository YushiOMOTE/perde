use crate::{
    error::Result,
    object::{ObjectRef, TupleIter, TupleRef},
};
use pyo3::ffi::*;
use std::iter::Enumerate;

pub struct Args<'a> {
    tuple: TupleRef<'a>,
}

impl<'a> Args<'a> {
    pub fn new(ptr: *mut PyObject) -> Result<Self> {
        Ok(Self {
            tuple: ObjectRef::new(ptr)?.as_tuple(),
        })
    }

    pub fn arg(&self, index: usize) -> Result<&ObjectRef> {
        self.tuple.get(index)
    }
}

pub struct FastArgs {
    args: *const *mut PyObject,
    kwnames: *mut PyObject,
    nargs: usize,
}

impl FastArgs {
    pub fn new(args: *const *mut PyObject, nargs: Py_ssize_t, kwnames: *mut PyObject) -> Self {
        let nargs = unsafe { PyVectorcall_NARGS(nargs) as usize };

        Self {
            args,
            kwnames,
            nargs,
        }
    }

    pub fn arg(&self, index: usize) -> Result<&ObjectRef> {
        if index >= self.num_args() {
            bail!(
                "missing {}-th positional argument; got {}",
                index,
                self.num_args()
            )
        }
        ObjectRef::new(unsafe { *self.args.offset(index as isize) })
    }

    pub fn iter_kwargs<'a>(&'a self) -> Result<Option<KwArgsIter<'a>>> {
        if self.kwnames.is_null() {
            return Ok(None);
        }

        let obj = ObjectRef::new(self.kwnames)?;

        Ok(Some(KwArgsIter {
            iter: obj.get_tuple_iter()?.enumerate(),
            args: self.args,
            nargs: self.nargs,
        }))
    }

    pub fn num_args(&self) -> usize {
        self.nargs
    }
}

pub struct KwArgsIter<'a> {
    iter: Enumerate<TupleIter<'a>>,
    args: *const *mut PyObject,
    nargs: usize,
}

impl<'a> Iterator for KwArgsIter<'a> {
    type Item = Result<(&'a str, &'a ObjectRef)>;

    fn next(&mut self) -> Option<Self::Item> {
        let (index, key) = self.iter.next()?;
        let key = match key.as_str() {
            Ok(key) => key,
            Err(e) => return Some(Err(e)),
        };
        let value =
            match ObjectRef::new(unsafe { *self.args.offset((self.nargs + index) as isize) }) {
                Ok(value) => value,
                Err(e) => return Some(Err(e)),
            };
        Some(Ok((key, value)))
    }
}
