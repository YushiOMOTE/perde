use crate::{
    error::Result,
    schema::*,
    types::{self, static_objects, AttrStr, Object, ObjectRef, StaticObject, TupleRef},
};
use indexmap::IndexMap;
use pyo3::ffi::PyObject;
use std::os::raw::c_char;

fn convert_stringcase(s: &str, case: Option<StrCase>) -> String {
    use inflections::Inflect;

    match case {
        Some(StrCase::Lower) => s.to_lower_case(),
        Some(StrCase::Upper) => s.to_upper_case(),
        Some(StrCase::Pascal) => s.to_pascal_case(),
        Some(StrCase::Camel) => s.to_camel_case(),
        Some(StrCase::Snake) => s.to_snake_case(),
        Some(StrCase::ScreamingSnake) => s.to_constant_case(),
        Some(StrCase::Kebab) => s.to_kebab_case(),
        Some(StrCase::ScreamingKebab) => s.to_kebab_case().to_upper_case(),
        None => s.into(),
    }
}

lazy_static::lazy_static! {
    static ref SCHEMA_CACHE: AttrStr = AttrStr::new("__perde_schema__");
    static ref DATACLASS_FIELDS: AttrStr = AttrStr::new("__dataclass_fields__");
    static ref ATTR_NAME: AttrStr = AttrStr::new("name");
    static ref ATTR_TYPE: AttrStr = AttrStr::new("type");
    static ref ATTR_VALUE: AttrStr = AttrStr::new("value");
    static ref ATTR_ARGS: AttrStr = AttrStr::new("__args__");
    static ref ATTR_ORIGIN: AttrStr = AttrStr::new("__origin__");
}

pub fn resolve_schema<'a>(p: &'a ObjectRef, attr: *mut PyObject) -> Result<&'a Schema> {
    match p.get_capsule(&SCHEMA_CACHE) {
        Ok(p) => return Ok(p),
        _ => {}
    }

    if p.is_bool() {
        Ok(&static_schema().boolean)
    } else if p.is_str() {
        Ok(&static_schema().string)
    } else if p.is_int() {
        Ok(&static_schema().int)
    } else if p.is_float() {
        Ok(&static_schema().float)
    } else if p.is_bytes() {
        Ok(&static_schema().bytes)
    } else if p.is_bytearray() {
        Ok(&static_schema().bytearray)
    } else if p.is_dict() {
        Ok(&static_schema().dict)
    } else if p.is_list() {
        Ok(&static_schema().list)
    } else if p.is_set() {
        Ok(&static_schema().set)
    } else if let Some(s) = maybe_dataclass(p, attr)? {
        p.set_capsule(&SCHEMA_CACHE, s)
    } else if let Some(s) = maybe_generic(p)? {
        p.set_capsule(&SCHEMA_CACHE, s)
    } else if let Some(s) = maybe_enum(p)? {
        p.set_capsule(&SCHEMA_CACHE, s)
    } else {
        bail!("unsupported type")
    }
}

pub fn to_schema(p: &ObjectRef) -> Result<Schema> {
    if p.is_bool() {
        Ok(Schema::Primitive(Primitive::Bool))
    } else if p.is_str() {
        Ok(Schema::Primitive(Primitive::Str))
    } else if p.is_int() {
        Ok(Schema::Primitive(Primitive::Int))
    } else if p.is_float() {
        Ok(Schema::Primitive(Primitive::Float))
    } else if p.is_bytes() {
        Ok(Schema::Primitive(Primitive::Bytes))
    } else if p.is_bytearray() {
        Ok(Schema::Primitive(Primitive::ByteArray))
    } else if let Some(s) = maybe_dataclass(p, std::ptr::null_mut())? {
        Ok(s)
    } else if let Some(s) = maybe_generic(p)? {
        Ok(s)
    } else if let Some(s) = maybe_enum(p)? {
        Ok(s)
    } else {
        bail!("unsupported type")
    }
}

fn maybe_dataclass(p: &ObjectRef, attr: *mut PyObject) -> Result<Option<Schema>> {
    if !p.has_attr(&DATACLASS_FIELDS) {
        return Ok(None);
    }

    let arg = types::Tuple::one(p)?;
    let fields = static_objects()?.fields.call(arg)?;
    let fields = types::Tuple::from(fields);

    let mut members = IndexMap::new();

    for i in 0..fields.len() {
        let field = fields.getref(i)?;
        let name = field.get_attr(&ATTR_NAME)?;
        let ty = field.get_attr(&ATTR_TYPE)?;

        let s = name.as_str()?;
        let mem = FieldSchema::new(
            AttrStr::new(s),
            i as usize,
            FieldAttr::default(),
            to_schema(ty.as_ref())?,
        );
        members.insert(s.to_string(), mem);
    }

    let name = p.name();
    let class = types::Class::new(p.to_owned());

    Ok(Some(Schema::Class(Class::new(
        class,
        name.into(),
        ClassAttr::default(),
        members,
        IndexMap::new(),
    ))))
}

fn maybe_enum(p: &ObjectRef) -> Result<Option<Schema>> {
    if !p.is_instance(static_objects()?.enum_meta.as_ptr()) {
        return Ok(None);
    }

    let iter = p.get_iter()?;

    let variants: Result<_> = iter
        .map(|item| {
            let name = item.get_attr(&ATTR_NAME)?;
            let value = item.get_attr(&ATTR_VALUE)?;

            let name = name.as_str()?;

            Ok((
                name.to_string(),
                VariantSchema::new(name.into(), VariantAttr::default(), value),
            ))
        })
        .collect();

    Ok(Some(Schema::Enum(Enum::new(
        EnumAttr::default(),
        variants?,
    ))))
}

fn maybe_option(args: TupleRef) -> Result<Schema> {
    let t1 = args.get(0)?;
    let t2 = args.get(1)?;
    let s = if t1.is_none_type() {
        let t = to_schema(t2)?;
        Schema::Optional(Optional::new(Box::new(t)))
    } else if t2.is_none_type() {
        let t = to_schema(t1)?;
        Schema::Optional(Optional::new(Box::new(t)))
    } else {
        Schema::Union(Union::new(vec![to_schema(t1)?, to_schema(t2)?]))
    };
    Ok(s)
}

fn to_union(p: &ObjectRef) -> Result<Schema> {
    let args = get_args(p)?;
    let args = args.as_ref();
    let iter = args.iter();

    if iter.len() == 2 {
        return maybe_option(args);
    }

    let variants: Result<Vec<_>> = iter.map(|arg| to_schema(arg)).collect();
    Ok(Schema::Union(Union::new(variants?)))
}

fn to_tuple(p: &ObjectRef) -> Result<Schema> {
    let args = get_args(p)?;
    let args = args.as_ref();
    let iter = args.iter();

    let args: Result<_> = iter.map(|arg| to_schema(arg)).collect();
    Ok(Schema::Tuple(Tuple::new(args?)))
}

fn to_dict(p: &ObjectRef) -> Result<Schema> {
    let args = get_args(p)?;
    let args = args.as_ref();
    let key = to_schema(args.get(0)?)?;
    let value = to_schema(args.get(1)?)?;
    Ok(Schema::Dict(Dict::new(Box::new(key), Box::new(value))))
}

fn to_list(p: &ObjectRef) -> Result<Schema> {
    let args = get_args(p)?;
    let args = args.as_ref();
    let value = to_schema(args.get(0)?)?;
    Ok(Schema::List(List::new(Box::new(value))))
}

fn to_set(p: &ObjectRef) -> Result<Schema> {
    let args = get_args(p)?;
    let args = args.as_ref();
    let value = to_schema(args.get(1)?)?;
    Ok(Schema::Set(Set::new(Box::new(value))))
}

fn get_args(p: &ObjectRef) -> Result<types::Tuple> {
    Ok(types::Tuple::from(p.get_attr(&ATTR_ARGS)?))
}

fn maybe_generic(p: &ObjectRef) -> Result<Option<Schema>> {
    if !p.is_instance(static_objects()?.generic_alias.as_ptr()) {
        return Ok(None);
    }

    let origin = p.get_attr(&ATTR_ORIGIN)?;

    let s = if origin.is(static_objects()?.union.as_ptr()) {
        to_union(p)?
    } else if origin.is(static_objects()?.tuple.as_ptr()) {
        to_tuple(p)?
    } else if origin.is_dict() {
        to_dict(p)?
    } else if origin.is_set() {
        to_set(p)?
    } else if origin.is_list() {
        to_list(p)?
    } else if origin.is_fronzen_set() {
        unimplemented!()
    } else {
        return Ok(None);
    };

    Ok(Some(s))
}
