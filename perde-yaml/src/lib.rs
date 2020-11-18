use perde_core::prelude::*;
use pyo3::ffi::*;

pyo3::create_exception!(perde_msgpack, YamlError, pyo3::exceptions::PyException);

pub extern "C" fn loads_as(_self: *mut PyObject, args: *mut PyObject) -> *mut PyObject {
    let inner = || {
        let args = TupleRef::from_args(args)?;

        let s = args.get(1).unwrap().as_str()?;
        let schema = args.get(0)?.resolve(None)?;
        let obj = serde_yaml::seed::from_str_seed(s, schema)?;

        Ok::<_, Error>(obj)
    };

    match inner() {
        Ok(p) => p.into_ptr(),
        Err(e) => {
            e.restore_as::<YamlError>();
            std::ptr::null_mut()
        }
    }
}

pub extern "C" fn dumps(
    _self: *mut PyObject,
    args: *const *mut PyObject,
    nargs: Py_ssize_t,
    kwnames: *mut PyObject,
) -> *mut PyObject {
    let inner = || {
        let fargs = FastArgs::new(args, nargs, kwnames);

        if fargs.num_args() != 1 {
            bail!("dumps() requires 1 positional argument");
        }

        let obj = fargs.arg(0)?;
        let resolved = obj.resolved_object()?;

        let s = serde_yaml::to_string(&resolved)?;

        Ok::<_, Error>(Object::new_str(&s)?)
    };

    match inner() {
        Ok(p) => p.into_ptr(),
        Err(e) => {
            e.restore_as::<YamlError>();
            std::ptr::null_mut()
        }
    }
}

pub extern "C" fn loads(
    _self: *mut PyObject,
    args: *const *mut PyObject,
    nargs: Py_ssize_t,
    kwnames: *mut PyObject,
) -> *mut PyObject {
    let inner = || {
        let fargs = FastArgs::new(args, nargs, kwnames);
        let obj = fargs.arg(0)?;
        let s: Object = serde_yaml::from_str(obj.as_str()?)?;
        Ok::<_, Error>(s)
    };

    match inner() {
        Ok(p) => p.into_ptr(),
        Err(e) => {
            e.restore_as::<YamlError>();
            std::ptr::null_mut()
        }
    }
}

module!(
    perde_yaml,
    exception!(YamlError),
    method_fastcall!(loads, ""),
    method_fastcall!(dumps, ""),
    method_varargs!(loads_as, "")
);
