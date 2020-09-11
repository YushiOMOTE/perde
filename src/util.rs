use pyo3::{exceptions, prelude::*};

pub type IndexMap<K, V> = indexmap::IndexMap<K, V, fnv::FnvBuildHasher>;

pub fn new_indexmap<K, V>() -> IndexMap<K, V> {
    IndexMap::with_hasher(fnv::FnvBuildHasher::default())
}

pub fn py<'a>() -> Python<'a> {
    unsafe { Python::assume_gil_acquired() }
}

pub fn pyerr<E: ToString>(e: E) -> PyErr {
    let py = py();

    if PyErr::occurred(py) {
        PyErr::fetch(py)
    } else {
        PyErr::new::<exceptions::RuntimeError, _>(e.to_string())
    }
}

pub fn de<E>(e: PyErr) -> E
where
    E: serde::de::Error,
{
    e.restore(py());
    serde::de::Error::custom("Python error")
}

pub fn ser<E>(e: PyErr) -> E
where
    E: serde::ser::Error,
{
    e.restore(py());
    serde::ser::Error::custom("Python error")
}
