use crate::{types::Object, util::*};
use pyo3::{prelude::*, wrap_pyfunction};

mod de;
mod types;
mod util;

macro_rules! load {
    ($a:tt, $f:tt, $d:tt) => {
        #[cfg(feature = $a)]
        #[pyfunction]
        pub fn $f(s: &str) -> PyObject {
            let obj: Object = $d::from_str(s).unwrap_or_else(restore_fin);
            obj.into()
        }
    };
}

load!("json", json_load, serde_json);
load!("yaml", yaml_load, serde_yaml);
load!("toml", toml_load, serde_toml);

#[cfg(feature = "msgpack")]
#[pyfunction]
pub fn msgpack_load(s: &[u8]) -> PyObject {
    let obj: Object = rmp_serde::from_slice(s).unwrap_or_else(restore_fin);
    obj.into()
}

#[pymodule]
fn serde_pyobj(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
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
