use crate::{
    error::{Convert, Error},
    schema::Schema,
    types::{self, Object, ObjectRef, TupleRef, _PyCFunctionFastWithKeywords},
};
use pyo3::{
    ffi::*, proc_macro::pymodule, types::PyModule, wrap_pyfunction, wrap_pymodule, PyResult, Python,
};
use serde::ser::Serialize;
use serde::Deserialize;
use std::os::raw::c_char;

pub unsafe extern "C" fn loads_as(_self: *mut PyObject, args: *mut PyObject) -> *mut PyObject {
    let inner = || {
        let args = TupleRef::from_args(args)?;

        let s = args.get(1).unwrap().as_str()?;
        let mut deserializer = serde_json::Deserializer::from_str(s);

        use serde::de::DeserializeSeed;
        let schema = args.get(0)?.resolve(None)?;
        let obj = schema.deserialize(&mut deserializer)?;

        Ok::<_, Error>(obj.into_ptr())
    };

    inner().restore().unwrap_or(std::ptr::null_mut())
}

pub unsafe extern "C" fn dumps(
    _self: *mut PyObject,
    args: *mut PyObject,
    // args: *const *mut PyObject,
    // nargs: Py_ssize_t,
    // kwnames: *mut PyObject,
) -> *mut PyObject {
    let inner = || {
        let args = TupleRef::from_args(args)?;
        let args = args.get(0).unwrap().as_ptr();

        let obj = unsafe {
            ObjectRef::new(
                args, // .offset(0)
            )?
        };

        let resolved = obj.resolved_object()?;

        let buf = vec![];
        let mut serializer = serde_json::Serializer::new(buf);
        resolved.serialize(&mut serializer);
        let buf = serializer.into_inner();

        assert!(!pyo3::PyErr::occurred(unsafe {
            pyo3::Python::assume_gil_acquired()
        }));

        Ok::<_, Error>(Object::new_str(&String::from_utf8(buf)?)?.into_ptr())
    };

    inner().restore().unwrap_or(std::ptr::null_mut())
}

pub unsafe extern "C" fn loads(
    _self: *mut PyObject,
    args: *const *mut PyObject,
    nargs: Py_ssize_t,
    kwnames: *mut PyObject,
) -> *mut PyObject {
    let inner = || {
        let obj = ObjectRef::new(*args.offset(0))?;
        let mut deserializer = serde_json::Deserializer::from_str(obj.as_str()?);
        Ok::<_, Error>(Object::deserialize(&mut deserializer).map(|v| v.into_ptr())?)
    };

    inner().restore().unwrap_or(std::ptr::null_mut())
}

#[pymodule]
pub fn json(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    use pyo3::AsPyPointer;

    let def = PyMethodDef {
        ml_name: "dumps\0".as_ptr() as *const c_char,
        ml_meth: Some(unsafe {
            dumps
            // std::mem::transmute::<_PyCFunctionFastWithKeywords, PyCFunction>(loads)
        }),
        ml_flags: METH_VARARGS, // METH_FASTCALL | METH_KEYWORDS,
        ml_doc: "".as_ptr() as *const c_char,
    };
    unsafe {
        PyModule_AddObject(
            m.as_ptr(),
            "dumps\0".as_ptr() as *const c_char,
            PyCFunction_NewEx(
                Box::into_raw(Box::new(def)),
                std::ptr::null_mut(),
                m.as_ptr(),
            ),
        )
    };

    let def = PyMethodDef {
        ml_name: "loads\0".as_ptr() as *const c_char,
        ml_meth: Some(unsafe {
            std::mem::transmute::<_PyCFunctionFastWithKeywords, PyCFunction>(loads)
        }),
        ml_flags: METH_FASTCALL | METH_KEYWORDS,
        ml_doc: "".as_ptr() as *const c_char,
    };
    unsafe {
        PyModule_AddObject(
            m.as_ptr(),
            "loads\0".as_ptr() as *const c_char,
            PyCFunction_New(Box::into_raw(Box::new(def)), std::ptr::null_mut()),
        )
    };

    let def = PyMethodDef {
        ml_name: "loads_as\0".as_ptr() as *const c_char,
        ml_meth: Some(loads_as),
        ml_flags: METH_VARARGS,
        ml_doc: "".as_ptr() as *const c_char,
    };
    unsafe {
        PyModule_AddObject(
            m.as_ptr(),
            "loads_as\0".as_ptr() as *const c_char,
            PyCFunction_New(Box::into_raw(Box::new(def)), std::ptr::null_mut()),
        )
    };

    Ok(())
}

pub fn import(m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(json))
}
