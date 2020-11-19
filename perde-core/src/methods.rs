pub type _PyCFunctionFastWithKeywords = unsafe extern "C" fn(
    slf: *mut pyo3::ffi::PyObject,
    args: *const *mut pyo3::ffi::PyObject,
    nargs: pyo3::ffi::Py_ssize_t,
    kwnames: *mut pyo3::ffi::PyObject,
) -> *mut pyo3::ffi::PyObject;

#[macro_export]
macro_rules! exception {
    ($exc:ident) => {
        |py: pyo3::Python<'_>, _: &str, m: &pyo3::types::PyModule| {
            m.add(stringify!($exc), py.get_type::<$exc>()).unwrap();
        }
    };
}

#[macro_export]
macro_rules! method_fastcall {
    ($method:ident, $doc:expr) => {
        |_: pyo3::Python<'_>, name: &str, m: &pyo3::types::PyModule| {
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
                    pyo3::ffi::PyCFunction_NewEx(
                        Box::into_raw(Box::new(def)),
                        std::ptr::null_mut(),
                        pyo3::ffi::PyUnicode_InternFromString(
                            name.as_ptr() as *const std::os::raw::c_char
                        ),
                    ),
                )
            };
        }
    };
}

#[macro_export]
macro_rules! method_varargs {
    ($method:ident, $doc:expr) => {
        |_: pyo3::Python<'_>, name: &str, m: &pyo3::types::PyModule| {
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
                    pyo3::ffi::PyCFunction_NewEx(
                        Box::into_raw(Box::new(def)),
                        std::ptr::null_mut(),
                        pyo3::ffi::PyUnicode_InternFromString(
                            name.as_ptr() as *const std::os::raw::c_char
                        ),
                    ),
                )
            };
        }
    };
}

#[macro_export]
macro_rules! module {
    ($name:tt, $($cls:expr),*) => {
        #[pyo3::proc_macro::pymodule]
        pub fn $name(py: pyo3::Python<'_>, m: &pyo3::types::PyModule) -> pyo3::PyResult<()> {
            $({
                let method = $cls;
                method(py, concat!(stringify!($name), "\0"), m);
            })*

            Ok(())
        }

        pub fn import(m: &pyo3::types::PyModule) -> pyo3::PyResult<()> {
            m.add_wrapped(pyo3::wrap_pymodule!($name))
        }
    };
}

#[macro_export]
macro_rules! impl_default_methods {
    ($module_name:tt, $exception_type:tt, $loads_as:ident, $loads:ident, $dumps:ident) => {
        pyo3::create_exception!($module_name, $exception_type, pyo3::exceptions::PyException);

        pub extern "C" fn loads_as(
            _self: *mut pyo3::ffi::PyObject,
            args: *mut pyo3::ffi::PyObject,
        ) -> *mut pyo3::ffi::PyObject {
            use pyo3::ffi::*;

            let inner = || {
                let args = Args::new(args)?;
                $loads_as(args.arg(0)?.resolve(None)?, args.arg(1)?)
            };

            match inner() {
                Ok(p) => p.into_ptr(),
                Err(e) => {
                    e.restore_as::<$exception_type>();
                    std::ptr::null_mut()
                }
            }
        }

        pub extern "C" fn dumps(
            _self: *mut pyo3::ffi::PyObject,
            args: *const *mut pyo3::ffi::PyObject,
            nargs: pyo3::ffi::Py_ssize_t,
            kwnames: *mut pyo3::ffi::PyObject,
        ) -> *mut pyo3::ffi::PyObject {
            let inner = || {
                let args = FastArgs::new(args, nargs, kwnames);

                if args.num_args() != 1 {
                    bail!("dumps() requires 1 positional argument");
                }

                let obj = args.arg(0)?;
                let resolved = obj.resolved_object()?;

                $dumps(resolved)
            };

            match inner() {
                Ok(p) => p.into_ptr(),
                Err(e) => {
                    e.restore_as::<$exception_type>();
                    std::ptr::null_mut()
                }
            }
        }

        pub extern "C" fn loads(
            _self: *mut pyo3::ffi::PyObject,
            args: *const *mut pyo3::ffi::PyObject,
            nargs: pyo3::ffi::Py_ssize_t,
            kwnames: *mut pyo3::ffi::PyObject,
        ) -> *mut pyo3::ffi::PyObject {
            let inner = || {
                let args = FastArgs::new(args, nargs, kwnames);
                let obj = args.arg(0)?;
                $loads(obj)
            };

            match inner() {
                Ok(p) => p.into_ptr(),
                Err(e) => {
                    e.restore_as::<$exception_type>();
                    std::ptr::null_mut()
                }
            }
        }

        module!(
            $module_name,
            exception!($exception_type),
            method_fastcall!(loads, ""),
            method_fastcall!(dumps, ""),
            method_varargs!(loads_as, "")
        );
    };
}
