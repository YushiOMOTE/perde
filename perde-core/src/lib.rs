#[macro_use]
pub mod error;

pub mod methods;

mod resolve;

pub mod schema;

mod decode;
mod encode;

pub mod args;
mod attr;
mod import;
pub mod object;

pub mod prelude {
    pub use crate::args::{Args, FastArgs};
    pub use crate::error::{Convert, Error, Result};
    pub use crate::object::{Object, ObjectRef};
    pub use crate::schema::{Schema, WithSchema};
    pub use crate::{
        bail, exception, impl_default_methods, method_fastcall, method_varargs, module,
    };
}
