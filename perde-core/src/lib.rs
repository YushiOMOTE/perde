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
    pub use crate::error::{raise, Convert, Error, Result};
    pub use crate::object::{Object, ObjectRef};
    pub use crate::schema::{Schema, WithSchema};
    pub use crate::{
        bail, bail_type_err, bail_value_err, err, exception, impl_default_methods, method_fastcall,
        method_varargs, module, submodule, type_err, value_err,
    };
}
