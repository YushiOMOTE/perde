#[macro_use]
pub mod error;

#[macro_use]
pub mod types;

pub mod methods;

pub mod resolve;
pub mod schema;

pub mod decode;
pub mod encode;

pub mod import;

pub mod prelude {
    pub use crate::error::{Convert, Error, Result};
    pub use crate::schema::{Schema, WithSchema};
    pub use crate::types::{Args, FastArgs, Object, ObjectRef};
    pub use crate::{
        bail, exception, impl_default_methods, method_fastcall, method_varargs, module,
    };
}
