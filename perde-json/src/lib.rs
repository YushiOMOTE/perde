use perde_core::{
    bail,
    error::{Convert, Error},
    method_fastcall, method_varargs, module,
    types::{FastArgs, Object, TupleRef},
};
use pyo3::ffi::*;
use serde::ser::Serialize;
use serde::Deserialize;

pub extern "C" fn loads_as(_self: *mut PyObject, args: *mut PyObject) -> *mut PyObject {
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

        let buf = vec![];
        let mut serializer = serde_json::Serializer::new(buf);
        resolved.serialize(&mut serializer)?;
        let buf = serializer.into_inner();

        Ok::<_, Error>(Object::new_str(&String::from_utf8(buf)?)?.into_ptr())
    };

    inner().restore().unwrap_or(std::ptr::null_mut())
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
        let mut deserializer = serde_json::Deserializer::from_str(obj.as_str()?);
        Ok::<_, Error>(Object::deserialize(&mut deserializer).map(|v| v.into_ptr())?)
    };

    inner().restore().unwrap_or(std::ptr::null_mut())
}

module!(
    perde_json,
    method_fastcall!(loads, ""),
    method_fastcall!(dumps, ""),
    method_varargs!(loads_as, "")
);
