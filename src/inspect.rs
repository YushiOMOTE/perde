use crate::{
    schema::*,
    types::{self, static_objects, Object, ObjectRef, StaticObject, TupleRef},
    util::*,
};
use pyo3::{prelude::*, types::PyModule};
use std::os::raw::c_char;

#[cfg_attr(feature = "perf", flame)]
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

const SCHEMA_CACHE: &'static str = "__perde_schema__\0";

static mut BOOL: Schema = Schema::Primitive(Primitive::Bool);
static mut INT: Schema = Schema::Primitive(Primitive::Int);
static mut STR: Schema = Schema::Primitive(Primitive::Str);
static mut FLOAT: Schema = Schema::Primitive(Primitive::Float);
static mut BYTES: Schema = Schema::Primitive(Primitive::Bytes);
static mut BYTEARRAY: Schema = Schema::Primitive(Primitive::ByteArray);
// static mut DICT: Schema = Schema::Dict(Dict::new(
//     Box::new(Schema::Any(Any::new())),
//     Box::new(Schema::Any(Any::new())),
// ));

pub fn resolve_schema<'a>(p: ObjectRef<'a>) -> PyResult<&'a Schema> {
    match p.load_item(SCHEMA_CACHE) {
        Ok(p) => return Ok(p),
        _ => {}
    }

    if p.is_bool() {
        Ok(unsafe { &BOOL })
    } else if p.is_str() {
        Ok(unsafe { &STR })
    } else if p.is_int() {
        Ok(unsafe { &INT })
    } else if p.is_float() {
        Ok(unsafe { &FLOAT })
    } else if p.is_bytes() {
        Ok(unsafe { &BYTES })
    } else if p.is_bytearray() {
        Ok(unsafe { &BYTEARRAY })
    } else if let Some(s) = maybe_dataclass(p)? {
        p.store_item(SCHEMA_CACHE, s)
    } else if let Some(s) = maybe_generic(p)? {
        p.store_item(SCHEMA_CACHE, s)
    } else if let Some(s) = maybe_enum(p)? {
        p.store_item(SCHEMA_CACHE, s)
    } else {
        Err(pyerr("unsupported type"))
    }
}

pub fn to_schema(p: ObjectRef) -> PyResult<Schema> {
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
    } else if let Some(s) = maybe_dataclass(p)? {
        Ok(s)
    } else if let Some(s) = maybe_generic(p)? {
        Ok(s)
    } else if let Some(s) = maybe_enum(p)? {
        Ok(s)
    } else {
        Err(pyerr("unsupported type"))
    }
}

fn maybe_dataclass(p: ObjectRef) -> PyResult<Option<Schema>> {
    if !p.has_attr("__dataclass_fields__\0") {
        return Ok(None);
    }

    let arg = types::Tuple::one(p)?;
    let fields = static_objects()?.fields.call(arg)?;
    let fields = types::Tuple::from(fields);

    let mut members = new_indexmap();

    for i in 0..fields.len() {
        let field = fields.getref(i)?;
        let name = field.get_attr("name\0")?;
        let ty = field.get_attr("type\0")?;

        let s = name.as_str()?;
        let mem = FieldSchema::new(
            s.into(),
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
        new_indexmap(),
    ))))
}

fn maybe_enum(p: ObjectRef) -> PyResult<Option<Schema>> {
    if !p.is_instance(static_objects()?.enum_meta.as_ptr()) {
        return Ok(None);
    }

    let iter = p.get_iter()?;

    let variants: PyResult<_> = iter
        .map(|item| {
            let name = item.get_attr("name\0")?;
            let value = item.get_attr("value\0")?;

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

fn maybe_option(args: TupleRef) -> PyResult<Schema> {
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

fn to_union(p: ObjectRef) -> PyResult<Schema> {
    let args = get_args(p)?;
    let args = args.as_ref();
    let iter = args.iter();

    if iter.len() == 2 {
        return maybe_option(args);
    }

    let variants: PyResult<Vec<_>> = iter.map(|arg| to_schema(arg)).collect();
    Ok(Schema::Union(Union::new(variants?)))
}

fn to_tuple(p: ObjectRef) -> PyResult<Schema> {
    let args = get_args(p)?;
    let args = args.as_ref();
    let iter = args.iter();

    let args: PyResult<_> = iter.map(|arg| to_schema(arg)).collect();
    Ok(Schema::Tuple(Tuple::new(args?)))
}

fn to_dict(p: ObjectRef) -> PyResult<Schema> {
    let args = get_args(p)?;
    let args = args.as_ref();
    let key = to_schema(args.get(0)?)?;
    let value = to_schema(args.get(1)?)?;
    Ok(Schema::Dict(Dict::new(Box::new(key), Box::new(value))))
}

fn to_list(p: ObjectRef) -> PyResult<Schema> {
    let args = get_args(p)?;
    let args = args.as_ref();
    let value = to_schema(args.get(0)?)?;
    Ok(Schema::List(List::new(Box::new(value))))
}

fn to_set(p: ObjectRef) -> PyResult<Schema> {
    let args = get_args(p)?;
    let args = args.as_ref();
    let value = to_schema(args.get(1)?)?;
    Ok(Schema::Set(Set::new(Box::new(value))))
}

fn get_args(p: ObjectRef) -> PyResult<types::Tuple> {
    Ok(types::Tuple::from(p.get_attr("__args__\0")?))
}

fn maybe_generic(p: ObjectRef) -> PyResult<Option<Schema>> {
    println!("MAYBE ORIGIN! {}", p.name());

    if !p.is_instance(static_objects()?.generic_alias.as_ptr()) {
        return Ok(None);
    }

    let origin = p.get_attr("__origin__\0")?;

    println!("ORIGIN!");

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
