use pyo3::{exceptions, prelude::*, types::PyDict, wrap_pyfunction};
use serde::{
    de::{self, Deserializer, Error, MapAccess, Visitor},
    Deserialize,
};
use std::fmt;

fn py<'a>() -> Python<'a> {
    unsafe { Python::assume_gil_acquired() }
}

fn restore<T: Error>(e: PyErr) -> T {
    e.restore(py());
    Error::custom("python error")
}

fn restore_fin<T: ToString>(e: T) -> Object {
    if !PyErr::occurred(py()) {
        let e = exceptions::RuntimeError::py_err(e.to_string());
        e.restore(py());
    }
    Object::null()
}

struct Object {
    inner: PyObject,
}

impl Object {
    fn new<T: ToPyObject>(value: T) -> Self {
        Self {
            inner: value.to_object(py()),
        }
    }

    fn null() -> Self {
        Self {
            inner: ().to_object(py()),
        }
    }
}

impl ToPyObject for Object {
    fn to_object(&self, _py: Python) -> PyObject {
        self.inner.clone()
    }
}

struct ObjectVisitor;

impl<'de> Visitor<'de> for ObjectVisitor {
    type Value = Object;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Object::new(value))
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Object::new(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Object::new(value))
    }

    fn visit_i128<E>(self, value: i128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(value))
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Object::new(value))
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Object::new(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Object::new(value))
    }

    fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(value))
    }

    fn visit_f32<E>(self, value: f32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(value))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(value))
    }

    fn visit_char<E>(self, value: char) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(value as u32))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(value))
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(value))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(value))
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(value))
    }

    fn visit_borrowed_bytes<E>(self, value: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(value))
    }

    fn visit_byte_buf<E>(self, value: Vec<u8>) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::new(value))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::null())
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Object::deserialize(deserializer)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Object::null())
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        Object::deserialize(deserializer)
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let dict = PyDict::new(py());

        while let Some((key, value)) = access.next_entry()? {
            let key: String = key;
            let value: Object = value;
            dict.set_item(key, value.inner).map_err(restore)?;
        }

        Ok(Object::new(dict))
    }
}

impl<'de> Deserialize<'de> for Object {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(ObjectVisitor)
    }
}

macro_rules! load {
    ($a:tt, $f:tt, $d:tt) => {
        #[cfg(feature = $a)]
        #[pyfunction]
        pub fn $f(s: &str) -> PyObject {
            let obj: Object = $d::from_str(s).unwrap_or_else(restore_fin);
            obj.inner
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
    obj.inner
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
