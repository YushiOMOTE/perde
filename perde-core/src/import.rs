use crate::{
    error::Result,
    object::{Object, SyncObject},
};
use pyo3::prelude::*;

pub struct Import {
    pub fields: SyncObject,
    pub missing: SyncObject,
    pub generic_alias: SyncObject,
    pub base_generic_alias: Option<SyncObject>,
    pub union_generic_alias: Option<SyncObject>,
    pub special_generic_alias: Option<SyncObject>,
    pub types_generic_alias: Option<SyncObject>,
    pub type_var: SyncObject,
    pub any: SyncObject,
    pub union: SyncObject,
    pub tuple: SyncObject,
    pub empty_tuple: SyncObject,
    pub optional: SyncObject,
    pub dict: SyncObject,
    pub list: SyncObject,
    pub set: SyncObject,
    pub frozenset: SyncObject,
    pub enum_meta: SyncObject,
    pub datetime: SyncObject,
    pub date: SyncObject,
    pub time: SyncObject,
    pub decimal: SyncObject,
    pub uuid: SyncObject,
}

pub fn import() -> Result<&'static Import> {
    IMPORT.as_ref().map_err(|e| err!("{}", e))
}

macro_rules! getattr {
    ($module:expr, $name:expr) => {
        $module
            .getattr($name)
            .map(|p| {
                Object::new(pyo3::PyObject::from(p).into_ptr())
                    .unwrap()
                    .into()
            })
            .map_err(|_| err!(concat!("couldn't find function `", $name, "`")))
    };
}

lazy_static::lazy_static! {
    static ref IMPORT: Result<Import> = {
        use pyo3::{Python, types::PyModule};

        let gil = Python::acquire_gil();
        let py = gil.python();

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
        let types_ = import!("types")?;

        let fields = getattr!(dataclasses, "fields")?;
        let missing = getattr!(dataclasses, "MISSING")?;
        let generic_alias = getattr!(typing, "_GenericAlias")?;
        let types_generic_alias = getattr!(types_, "GenericAlias").ok();
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

        let empty_tuple = SyncObject::new(Object::new_unit()?);

        let datetime = getattr!(datetime_, "datetime")?;
        let date = getattr!(datetime_, "date")?;
        let time = getattr!(datetime_, "time")?;
        let decimal = getattr!(decimal_, "Decimal")?;
        let uuid = getattr!(uuid_, "UUID")?;

        Ok(Import {
            fields,
            missing,
            generic_alias,
            types_generic_alias,
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
