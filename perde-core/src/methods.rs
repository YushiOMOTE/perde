pub type _PyCFunctionFastWithKeywords = unsafe extern "C" fn(
    slf: *mut pyo3::ffi::PyObject,
    args: *const *mut pyo3::ffi::PyObject,
    nargs: pyo3::ffi::Py_ssize_t,
    kwnames: *mut pyo3::ffi::PyObject,
) -> *mut pyo3::ffi::PyObject;

#[macro_export]
macro_rules! method_fastcall {
    ($method:ident, $doc:expr) => {
        |m: &pyo3::types::PyModule| {
            let def = pyo3::ffi::PyMethodDef {
                ml_name: concat!(stringify!($method), "\0").as_ptr() as *const std::os::raw::c_char,
                ml_meth: Some(unsafe {
                    std::mem::transmute::<
                        $crate::methods::_PyCFunctionFastWithKeywords,
                        pyo3::ffi::PyCFunction,
                    >($method)
                }),
                ml_flags: pyo3::ffi::METH_FASTCALL | pyo3::ffi::METH_KEYWORDS,
                ml_doc: $doc.as_ptr() as *const std::os::raw::c_char,
            };
            unsafe {
                pyo3::ffi::PyModule_AddObject(
                    pyo3::conversion::AsPyPointer::as_ptr(m),
                    concat!(stringify!($method), "\0").as_ptr() as *const std::os::raw::c_char,
                    pyo3::ffi::PyCFunction_New(Box::into_raw(Box::new(def)), std::ptr::null_mut()),
                )
            };
        }
    };
}

#[macro_export]
macro_rules! method_varargs {
    ($method:ident, $doc:expr) => {
        |m: &pyo3::types::PyModule| {
            let def = pyo3::ffi::PyMethodDef {
                ml_name: concat!(stringify!($method), "\0").as_ptr() as *const std::os::raw::c_char,
                ml_meth: Some($method),
                ml_flags: pyo3::ffi::METH_VARARGS,
                ml_doc: $doc.as_ptr() as *const std::os::raw::c_char,
            };
            unsafe {
                pyo3::ffi::PyModule_AddObject(
                    pyo3::conversion::AsPyPointer::as_ptr(m),
                    concat!(stringify!($method), "\0").as_ptr() as *const std::os::raw::c_char,
                    pyo3::ffi::PyCFunction_New(Box::into_raw(Box::new(def)), std::ptr::null_mut()),
                )
            };
        }
    };
}

#[macro_export]
macro_rules! module {
    ($name:tt, $($cls:expr),*) => {
        #[pyo3::proc_macro::pymodule]
        pub fn $name(_py: pyo3::Python<'_>, m: &pyo3::types::PyModule) -> pyo3::PyResult<()> {
            $({
                let method = $cls;
                method(m);
            })*

            Ok(())
        }

        pub fn import(m: &pyo3::types::PyModule) -> pyo3::PyResult<()> {
            m.add_wrapped(pyo3::wrap_pymodule!($name))
        }
    };
}
