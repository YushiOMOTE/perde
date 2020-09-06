use crate::types::Schema;
use pyo3::{prelude::*, types::PyDict, wrap_pyfunction};

#[cfg(feature = "perf")]
#[macro_use]
extern crate flamer;

mod de;
#[cfg(feature = "json")]
mod json;
mod ser;
mod state;
mod types;
mod util;

#[pyfunction(kwargs = "**")]
pub fn resolve(ty: &PyAny, kwargs: Option<&PyDict>) -> PyResult<()> {
    Schema::resolve_with_attr(ty, kwargs)?;
    Ok(())
}

#[pymodule]
fn perde(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Schema>()?;

    #[cfg(feature = "json")]
    json::import(m)?;

    m.add_wrapped(wrap_pyfunction!(resolve))?;

    Ok(())
}
