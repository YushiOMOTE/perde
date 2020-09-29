#[cfg(feature = "json")]
mod json;
#[cfg(feature = "msgpack")]
mod msgpack;
#[cfg(feature = "yaml")]
mod yaml;

use pyo3::{prelude::*, types::PyModule};

pub fn import(m: &PyModule) -> PyResult<()> {
    #[cfg(feature = "json")]
    self::json::import(m)?;
    #[cfg(feature = "yaml")]
    self::yaml::import(m)?;
    #[cfg(feature = "msgpack")]
    self::msgpack::import(m)?;

    Ok(())
}
