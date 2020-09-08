use crate::{object::Object, schema::Schema, util::*};
use pyo3::{prelude::*, wrap_pyfunction, wrap_pymodule};
use serde::Deserialize;
use std::os::raw::c_char;

#[pyfunction]
pub fn loads_as(ty: &PyAny, s: &str) -> PyResult<PyObject> {
    let mut deserializer = serde_json::Deserializer::from_str(s);
    let obj = Schema::deserialize(ty, &mut deserializer)?;

    #[cfg(feature = "perf")]
    {
        flame::dump_html(&mut std::fs::File::create("flame-graph.html").unwrap()).unwrap();
        flame::clear();
    }

    Ok(obj)
}

#[pyfunction]
pub fn dumps(v: &PyAny) -> PyResult<String> {
    let buf = vec![];
    let mut serializer = serde_json::Serializer::new(buf);
    Schema::serialize(v, &mut serializer)?;
    let buf = serializer.into_inner();

    #[cfg(feature = "perf")]
    {
        flame::dump_html(&mut std::fs::File::create("flame-graph.html").unwrap()).unwrap();
        flame::clear();
    }

    Ok(String::from_utf8(buf)?)
}

// #[pyfunction(kwargs = "**")]
// #[pyfunction]
// pub fn loads(p: &PyAny) -> PyResult<PyObject> {
//     use pyo3::AsPyPointer;
//     let mut size = 0;
//     let p = crate::unicode::read_utf8_from_str(p.as_ptr(), &mut size);
//     let s = unsafe { std::slice::from_raw_parts(p, size as usize) };
//     Ok(1.to_object(py()))
//     // let mut deserializer = serde_json::Deserializer::from_slice(s);
//     // Object::deserialize(&mut deserializer)
//     //     .map_err(pyerr)
//     //     .map(|v| v.to_pyobj())
// }

// pub fn loads(p: &PyAny) -> PyResult<PyObject> {
//     use pyo3::AsPyPointer;
//     let mut size = 0;
//     let p = crate::unicode::read_utf8_from_str(p.as_ptr(), &mut size);
//     let s = unsafe { std::slice::from_raw_parts(p, size as usize) };
//     Ok(1.to_object(py()))
// }

pub unsafe extern "C" fn loads(
    _self: *mut pyo3::ffi::PyObject,
    obj: *mut pyo3::ffi::PyObject,
) -> *mut pyo3::ffi::PyObject {
    use pyo3::AsPyPointer;
    let mut size = 0;
    let p = crate::unicode::read_utf8_from_str(obj, &mut size);
    let s = unsafe { std::slice::from_raw_parts(p, size as usize) };
    let mut deserializer = serde_json::Deserializer::from_slice(s);
    Object::deserialize(&mut deserializer)
        .map_err(pyerr)
        .map(|v| v.to_pyobj().as_ptr())
        .unwrap_or_else(|_| std::ptr::null_mut())
    // 1.to_object(py()).as_ptr()
    // Ok(1.to_object(py()))
}

// fn __pyo3_get_function_loads(py: pyo3::Python) -> pyo3::PyObject {
//     unsafe extern "C" fn __wrap(
//         _slf: *mut pyo3::ffi::PyObject,
//         _args: *mut pyo3::ffi::PyObject,
//         _kwargs: *mut pyo3::ffi::PyObject,
//     ) -> *mut pyo3::ffi::PyObject {
//         const _LOCATION: &'static str = "loads()";
//         {
//             {
//                 let pool = ::pyo3::GILPool::new();
//                 let unwind_safe_py = std::panic::AssertUnwindSafe(pool.python());
//                 let result = match std::panic::catch_unwind(move || -> ::pyo3::PyResult<_> {
//                     let _py = *unwind_safe_py;
//                     ::pyo3::callback::convert(_py, {
//                         let _args = _py.from_borrowed_ptr::<pyo3::types::PyTuple>(_args);
//                         let _kwargs: Option<&pyo3::types::PyDict> =
//                             _py.from_borrowed_ptr_or_opt(_kwargs);
//                         {
//                             const PARAMS: &'static [pyo3::derive_utils::ParamDescription] =
//                                 &[pyo3::derive_utils::ParamDescription {
//                                     name: "p",
//                                     is_optional: false,
//                                     kw_only: false,
//                                 }];
//                             let mut output = [None; 1usize];
//                             let mut _args = _args;
//                             let mut _kwargs = _kwargs;
//                             let (_args, _kwargs) = pyo3::derive_utils::parse_fn_args(
//                                 Some(_LOCATION),
//                                 PARAMS,
//                                 _args,
//                                 _kwargs,
//                                 false,
//                                 false,
//                                 &mut output,
//                             )?;
//                             let _tmp: <&PyAny as pyo3::derive_utils::ExtractExt>::Target =
//                                 output[0usize].unwrap().extract()?;
//                             let arg0 = &*_tmp;
//                             loads(arg0)
//                         }
//                     })
//                 }) {
//                     Ok(result) => result,
//                     Err(e) => {
//                         if let Some(string) = e.downcast_ref::<String>() {
//                             Err(::pyo3::panic::PanicException::py_err((string.clone(),)))
//                         } else if let Some(s) = e.downcast_ref::<&str>() {
//                             Err(::pyo3::panic::PanicException::py_err((s.to_string(),)))
//                         } else {
//                             Err(::pyo3::panic::PanicException::py_err((
//                                 "panic from Rust code",
//                             )))
//                         }
//                     }
//                 };
//                 result.unwrap_or_else(|e| {
//                     e.restore(pool.python());
//                     ::pyo3::callback::callback_error()
//                 })
//             }
//         }
//     }
//     let _def = pyo3::class::PyMethodDef {
//         ml_name: "loads",
//         ml_meth: pyo3::class::PyMethodType::PyCFunctionWithKeywords(__wrap),
//         ml_flags: pyo3::ffi::METH_VARARGS | pyo3::ffi::METH_KEYWORDS,
//         ml_doc: "\u{0}",
//     };
//     let function = unsafe {
//         pyo3::PyObject::from_owned_ptr_or_panic(
//             py,
//             pyo3::ffi::PyCFunction_New(
//                 Box::into_raw(Box::new(_def.as_method_def())),
//                 ::std::ptr::null_mut(),
//             ),
//         )
//     };
//     function
// }

#[pymodule]
pub fn json(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    use pyo3::AsPyPointer;

    m.add_wrapped(wrap_pyfunction!(loads_as))?;
    // m.add_wrapped(&__pyo3_get_function_loads)?;
    m.add_wrapped(wrap_pyfunction!(dumps))?;

    let def = pyo3::ffi::PyMethodDef {
        ml_name: "loads\0".as_ptr() as *const c_char,
        ml_meth: Some(loads),
        ml_flags: pyo3::ffi::METH_O,
        ml_doc: "".as_ptr() as *const c_char,
    };

    unsafe {
        pyo3::ffi::PyModule_AddObject(
            m.as_ptr(),
            "loads\0".as_ptr() as *const c_char,
            pyo3::ffi::PyCFunction_New(Box::into_raw(Box::new(def)), std::ptr::null_mut()),
        )
    };

    Ok(())
}

pub fn import(m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(json))
}
