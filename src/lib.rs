// use crate::schema::Schema;
use pyo3::{prelude::*, types::PyDict, wrap_pyfunction};

#[cfg(feature = "perf")]
#[macro_use]
extern crate flamer;

#[macro_use]
mod types;
mod util;

mod inspect;
mod schema;

mod decode;
// mod encode;

// #[cfg(feature = "json")]
// mod json;
// mod object;

// mod unicode;

// #[pyfunction(kwargs = "**")]
// pub fn resolve(ty: &PyAny, kwargs: Option<&PyDict>) -> PyResult<()> {
//     Schema::resolve(ty, kwargs)?;
//     Ok(())
// }

// #[pymodule]
// fn perde(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
//     #[cfg(feature = "json")]
//     json::import(m)?;

//     m.add_wrapped(wrap_pyfunction!(resolve))?;

//     Ok(())
// }
