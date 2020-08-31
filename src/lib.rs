use crate::{
    types::{register_py_schema, Object, PySchema},
    util::*,
};
use pyo3::{prelude::*, wrap_pyfunction};

mod de;
mod types;
mod util;

macro_rules! load {
    ($a:tt, $f:tt, $d:tt) => {
        #[cfg(feature = $a)]
        #[pyfunction]
        pub fn $f(s: &str) -> PyResult<PyObject> {
            let obj: Object = $d::from_str(s).map_err(pyerr)?;
            Ok(obj.into())
        }
    };
}

load!("json", json_load, serde_json);
load!("yaml", yaml_load, serde_yaml);
load!("toml", toml_load, serde_toml);

#[pyfunction]
pub fn register(key: u64, schema: PySchema) -> PyResult<()> {
    register_py_schema(key, &schema)
}

#[cfg(feature = "msgpack")]
#[pyfunction]
pub fn msgpack_load(s: &[u8]) -> PyResult<PyObject> {
    let obj: Object = rmp_serde::from_slice(s).map_err(pyerr)?;
    Ok(obj.into())
}

#[pymodule]
fn perde(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySchema>()?;

    m.add_wrapped(wrap_pyfunction!(register))?;

    #[cfg(feature = "json")]
    m.add_wrapped(wrap_pyfunction!(json_load))?;
    #[cfg(feature = "yaml")]
    m.add_wrapped(wrap_pyfunction!(yaml_load))?;
    #[cfg(feature = "toml")]
    m.add_wrapped(wrap_pyfunction!(toml_load))?;
    #[cfg(feature = "msgpack")]
    m.add_wrapped(wrap_pyfunction!(msgpack_load))?;

    Ok(())
}
