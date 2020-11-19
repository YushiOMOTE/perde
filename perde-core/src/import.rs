use crate::{error::Result, object::Object};
use pyo3::prelude::*;
use std::ops::Deref;

#[derive(Debug)]
pub struct StaticObject(Object);

impl Deref for StaticObject {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<pyo3::PyObject> for StaticObject {
    fn from(p: pyo3::PyObject) -> Self {
        StaticObject(Object::new(p.into_ptr()).unwrap())
    }
}

pub struct Import {
    pub fields: StaticObject,
    pub missing: StaticObject,
    pub generic_alias: StaticObject,
    pub base_generic_alias: Option<StaticObject>,
    pub union_generic_alias: Option<StaticObject>,
    pub special_generic_alias: Option<StaticObject>,
    pub type_var: StaticObject,
    pub any: StaticObject,
    pub union: StaticObject,
    pub tuple: StaticObject,
    pub empty_tuple: StaticObject,
    pub optional: StaticObject,
    pub dict: StaticObject,
    pub list: StaticObject,
    pub set: StaticObject,
    pub frozenset: StaticObject,
    pub enum_meta: StaticObject,
    pub datetime: StaticObject,
    pub date: StaticObject,
    pub time: StaticObject,
    pub decimal: StaticObject,
    pub uuid: StaticObject,
}

unsafe impl Sync for StaticObject {}

pub fn import() -> Result<&'static Import> {
    STATIC_OBJECTS.as_ref().map_err(|e| err!("{}", e))
}

macro_rules! getattr {
    ($module:expr, $name:expr) => {
        $module
            .getattr($name)
            .map(|p| pyo3::PyObject::from(p).into())
            .map_err(|_| err!(concat!("couldn't find function `", $name, "`")))
    };
}

lazy_static::lazy_static! {
    static ref STATIC_OBJECTS: Result<Import> = {
        use pyo3::{Python, types::PyModule};

        let py = unsafe { Python::assume_gil_acquired() };

        macro_rules! import {
            ($name:expr) => {
                PyModule::import(py, $name)
                    .map_err(|_| err!("couldn't import `{}`", $name))
            };
        }

        let dataclasses = import!("dataclasses")?;
        let typing = import!("typing")?;
        let enum_ = import!("enum")?;
        let datetime_ = import!("datetime")?;
        let decimal_ = import!("decimal")?;
        let uuid_ = import!("uuid")?;

        let fields = getattr!(dataclasses, "fields")?;
        let missing = getattr!(dataclasses, "MISSING")?;
        let generic_alias = getattr!(typing, "_GenericAlias")?;
        let union_generic_alias = getattr!(typing, "_UnionGenericAlias").ok();
        let base_generic_alias = getattr!(typing, "_BaseGenericAlias").ok();
        let special_generic_alias = getattr!(typing, "_SpecialGenericAlias").ok();
        let type_var = getattr!(typing, "TypeVar")?;
        let any = getattr!(typing, "Any")?;
        let union = getattr!(typing, "Union")?;
        let tuple = getattr!(typing, "Tuple")?;
        let optional = getattr!(typing, "Optional")?;
        let dict = getattr!(typing, "Dict")?;
        let list = getattr!(typing, "List")?;
        let set = getattr!(typing, "Set")?;
        let frozenset = getattr!(typing, "FrozenSet")?;
        let enum_meta = getattr!(enum_, "EnumMeta")?;

        let empty_tuple = StaticObject(Object::new_unit()?);

        let datetime = getattr!(datetime_, "datetime")?;
        let date = getattr!(datetime_, "date")?;
        let time = getattr!(datetime_, "time")?;
        let decimal = getattr!(decimal_, "Decimal")?;
        let uuid = getattr!(uuid_, "UUID")?;

        Ok(Import {
            fields,
            missing,
            generic_alias,
            union_generic_alias,
            base_generic_alias,
            special_generic_alias,
            type_var,
            any,
            union,
            tuple,
            empty_tuple,
            optional,
            dict,
            list,
            set,
            frozenset,
            enum_meta,
            datetime,
            date,
            time,
            decimal,
            uuid,
        })
    };
}
