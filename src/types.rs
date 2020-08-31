use crate::util::*;
use lazy_static::lazy_static;
use pyo3::{
    prelude::*,
    types::{PyDict, PyList, PySet, PyTuple, PyType},
};
use std::{
    cell::RefCell,
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct Object {
    inner: PyObject,
}

impl Object {
    pub fn new<T: ToPyObject>(value: T) -> Self {
        Self {
            inner: value.to_object(py()),
        }
    }

    pub fn null() -> Self {
        Self {
            inner: ().to_object(py()),
        }
    }

    pub fn to_pyobj(self) -> PyObject {
        self.inner
    }
}

impl From<Object> for PyObject {
    fn from(obj: Object) -> Self {
        obj.to_pyobj()
    }
}

pub struct Str;

impl Str {
    pub fn create(&self, s: &str) -> PyObject {
        s.to_object(py())
    }
}

pub struct Bytes;

impl Bytes {
    pub fn create(&self, s: &[u8]) -> PyObject {
        s.to_object(py())
    }
}

pub struct Bool;

impl Bool {
    pub fn create(&self, b: bool) -> PyObject {
        b.to_object(py())
    }
}

pub struct Int;

impl Int {
    pub fn create(&self, i: i64) -> PyObject {
        i.to_object(py())
    }
}

pub struct Float;

impl Float {
    pub fn create(&self, f: f64) -> PyObject {
        f.to_object(py())
    }
}

pub struct List {
    value: Arc<Schema>,
}

impl List {
    pub fn create<I, T, U>(&self, args: I) -> PyObject
    where
        I: IntoIterator<Item = T, IntoIter = U>,
        T: ToPyObject,
        U: ExactSizeIterator<Item = T>,
    {
        PyList::new(py(), args).into()
    }

    pub fn value_type(&self) -> &Arc<Schema> {
        &self.value
    }
}

pub struct Tuple {
    values: Vec<Arc<Schema>>,
}

impl Tuple {
    pub fn create<I, T, U>(&self, args: I) -> PyObject
    where
        I: IntoIterator<Item = T, IntoIter = U>,
        T: ToPyObject,
        U: ExactSizeIterator<Item = T>,
    {
        PyTuple::new(py(), args).into()
    }

    pub fn value_types(&self) -> &[Arc<Schema>] {
        &self.values
    }
}

struct Set {
    value: Arc<Schema>,
}

impl Set {
    pub fn create(&self, args: &[PyObject]) -> PyResult<PyObject> {
        Ok(PySet::new(py(), args)?.into())
    }

    pub fn value_type(&self) -> &Arc<Schema> {
        &self.value
    }
}

struct Dict {
    values: HashMap<String, Arc<Schema>>,
}

impl Dict {
    pub fn create(&self, args: PyObject) -> PyResult<PyObject> {
        Ok(PyDict::from_sequence(py(), args)?.into())
    }

    pub fn value_types(&self) -> &HashMap<String, Arc<Schema>> {
        &self.values
    }
}

struct Class {
    pytype: Py<PyType>,
    values: HashMap<String, Arc<Schema>>,
}

impl Class {
    pub fn create(&self, args: &PyDict) -> PyResult<PyObject> {
        self.pytype
            .as_ref(py())
            .call((), Some(args))
            .map(|p| p.into())
    }

    pub fn value_types(&self) -> &HashMap<String, Arc<Schema>> {
        &self.values
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct PySchema {
    cls: Py<PyType>,
    args: Vec<PySchema>,
    kwargs: HashMap<String, PySchema>,
    attr: Vec<String>,
}

#[pymethods]
impl PySchema {
    #[new]
    fn new(
        cls: Py<PyType>,
        args: Vec<PySchema>,
        kwargs: HashMap<String, PySchema>,
        attr: Vec<String>,
    ) -> Self {
        Self {
            cls,
            args,
            kwargs,
            attr,
        }
    }
}

struct PySchemaStack<'a> {
    stack: Vec<&'a PySchema>,
}

impl<'a> PySchemaStack<'a> {
    fn new(schema: &'a PySchema) -> Self {
        Self {
            stack: vec![schema],
        }
    }

    fn current(&self) -> PyResult<&'a PySchema> {
        self.stack
            .last()
            .map(|p| *p)
            .ok_or_else(|| pyerr("Schema stack is empty"))
    }

    fn push_by_name(&mut self, name: &str) -> PyResult<()> {
        let cur = self
            .stack
            .last()
            .ok_or_else(|| pyerr("Schema stack is empty"))?;
        let next = cur
            .kwargs
            .get(name)
            .ok_or_else(|| pyerr(format!("Couldn't find field with name: {}", name)))?;
        self.stack.push(next);
        Ok(())
    }

    fn push_by_index(&mut self, index: usize) -> PyResult<()> {
        let cur = self
            .stack
            .last()
            .ok_or_else(|| pyerr("Schema stack is empty"))?;
        let next = cur
            .args
            .get(index)
            .ok_or_else(|| pyerr(format!("Couldn't find field with index: {}", index)))?;
        self.stack.push(next);
        Ok(())
    }

    fn pop(&mut self) {
        self.stack.pop();
    }
}

pub enum Schema {
    None,
    Str(Str),
    Bytes(Bytes),
    Bool(Bool),
    Int(Int),
    Float(Float),
    List(List),
    Tuple(Tuple),
    Set(Set),
    Dict(Dict),
    Class(Class),
}

thread_local! {
    pub static SCHEMA: RefCell<Vec<Arc<Schema>>> = RefCell::new(vec![]);
}

lazy_static! {
    static ref SCHEMA_REGISTRY: RwLock<HashMap<u64, Arc<Schema>>> = { RwLock::new(HashMap::new()) };
}

fn to_schema(schema: &PySchema) -> PyResult<Schema> {
    // TODO
    println!("pyschema = {:#?}", schema);
    Ok(Schema::None)
}

pub fn register_py_schema(key: u64, schema: &PySchema) -> PyResult<()> {
    println!("register python type: key={}, schema={:?}", key, schema);
    Ok(register_schema(key, to_schema(schema)?))
}

pub fn register_schema(key: u64, schema: Schema) {
    SCHEMA_REGISTRY
        .write()
        .unwrap()
        .insert(key, Arc::new(schema));
}

pub fn get_schema(key: u64) -> Option<Arc<Schema>> {
    SCHEMA_REGISTRY.read().unwrap().get(&key).map(|v| v.clone())
}

// #[derive(Debug, Clone, Copy)]
// enum TypeKind {
//     Str,
//     Bytes,
//     Bool,
//     Int,
//     Float,
//     Seq,
//     Dict,
// }

// #[derive(Debug, Clone)]
// pub struct Schema {
//     obj: Py<PyType>,
//     kind: TypeKind,
//     children: Vec<RefCell<Schema>>,
// }

// pub fn kind(obj: &PyType) -> PyResult<Type> {
//     if py().is_instance::<pyo3::types::PyUnicode, _>(obj)? {
//         Ok(Type::Str)
//     } else {
//         unimplemented!()
//     }
// }

// impl Schema {
//     pub fn new(obj: PyObject) -> PyResult<Self> {
//         let py = py();
//         let obj: &PyType = obj.cast_as(py)?;
//         let kind = kind(obj)?;

//         match kind {
//             TypeKind::Dict => {
//                 let dict: PyDict = obj.cast_as(py)?;

//                 Ok(Self {
//                     obj: obj.into(),
//                     kind,
//                     children,
//                 })
//             }
//             TypeKind::Seq => unimplemented!(),
//             _ => Ok(Self {
//                 obj: obj.into(),
//                 kind,
//                 children: vec![],
//             }),
//         }
//     }

//     pub fn create<T: ToPyObject>(&self, value: T) -> PyResult<Object> {
//         let py = py();
//         let value: PyObject = self.inner.call1(py, (value.to_object(py),))?;
//         Ok(Object::new(value))
//     }

//     pub fn create_class(&self, args: &PyDict) -> PyResult<Object> {
//         let obj: PyObject = self.inner.call(py(), (), Some(args))?;
//         Ok(Object::new(obj))
//     }

//     pub fn get(&self, name: &str) -> PyResult<Schema> {
//         let dict: &PyDict = self.inner.cast_as(py())?;
//         let item = dict
//             .get_item(name)
//             .ok_or_else(|| pyerr(format!("No such item in dict: {}", name)))?
//             .to_object(py());
//         Schema::new(item)
//     }
// }

// thread_local! {
//     pub static SCHEMA: RefCell<Vec<Schema>> = RefCell::new(vec![]);
// }

// pub fn setup_schema(schema: Schema) {
//     SCHEMA.with(move |s| {
//         s.borrow_mut().push(schema);
//     })
// }

// pub fn schema_push(name: &str) -> PyResult<()> {
//     SCHEMA.with(move |s| {
//         let mut s = s.borrow_mut();
//         let schema = s.last().ok_or_else(|| pyerr("No parent schema found"))?;
//         let schema = schema.get(name)?;
//         s.push(schema);
//         Ok(())
//     })
// }

// pub fn with_schema<F: FnOnce(&Schema) -> R, R>(f: F) -> R {
//     SCHEMA.with(|s| f(s.borrow().last().expect("Schema must exist")))
// }

// pub fn schema_pop() {
//     SCHEMA.with(move |s| {
//         s.borrow_mut().pop();
//     });
// }
