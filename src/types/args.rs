use crate::{
    error::Result,
    types::{ObjectRef, TupleRef, TupleRefIter},
};
use pyo3::ffi::*;
use std::iter::Enumerate;

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
        ObjectRef::new(unsafe { *self.args.offset(0) })
    }

    pub fn iter_kwargs<'a>(&'a self) -> Result<Option<KwArgsIter<'a>>> {
        if self.kwnames.is_null() {
            return Ok(None);
        }

        Ok(Some(KwArgsIter {
            iter: TupleRef::new(ObjectRef::new(self.kwnames)?)
                .iter()
                .enumerate(),
            args: self.args,
            nargs: self.nargs,
        }))
    }

    pub fn num_args(&self) -> usize {
        self.nargs
    }
}

pub struct KwArgsIter<'a> {
    iter: Enumerate<TupleRefIter<'a>>,
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
