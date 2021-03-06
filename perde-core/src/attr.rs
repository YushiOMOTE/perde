use std::{
    fmt::{self, Display},
    os::raw::c_char,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttrStr(String);

impl AttrStr {
    pub fn new(s: &str) -> Self {
        Self(format!("{}\0", s))
    }

    pub fn as_ptr(&self) -> *mut c_char {
        self.0.as_ptr() as *mut c_char
    }
}

impl Display for AttrStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}
