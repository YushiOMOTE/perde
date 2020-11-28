use crate::{
    attr::AttrStr,
    error::Convert,
    error::Result,
    import::import,
    object::{ObjectRef, TupleIter},
    schema::*,
};
use indexmap::IndexMap;
use std::{borrow::Cow, collections::HashMap};

fn collect_members(
    mems: &IndexMap<String, FieldSchema>,
) -> (IndexMap<String, FieldSchema>, bool, usize) {
    let mut has_flatten = false;
    let mut skip_field_len = 0;

    let mems = mems
        .iter()
        .flat_map(|(key, field)| {
            if field.attr.flatten {
                if let Schema::Class(cls) = &field.schema {
                    has_flatten = true;
                    return collect_members(&cls.fields).0;
                }
            } else if field.attr.skip || field.attr.skip_serializing {
                skip_field_len += 1;
            }
            let mut map = IndexMap::new();
            map.insert(key.to_string(), field.clone());
            map
        })
        .collect();

    (mems, has_flatten, skip_field_len)
}

fn collect_flatten_members(
    mems: &IndexMap<String, FieldSchema>,
) -> (IndexMap<String, FieldSchema>, usize) {
    let (mems, has_flatten, skip_len) = collect_members(mems);
    if has_flatten {
        (mems, skip_len)
    } else {
        (IndexMap::new(), 0)
    }
}

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
    static ref ATTR_DEFAULT: AttrStr = AttrStr::new("default");
    static ref ATTR_DEFAULT_FACTORY: AttrStr = AttrStr::new("default_factory");
    static ref ATTR_METADATA: AttrStr = AttrStr::new("metadata");
    static ref ATTR_VALUE: AttrStr = AttrStr::new("value");
    static ref ATTR_ARGS: AttrStr = AttrStr::new("__args__");
    static ref ATTR_ORIGIN: AttrStr = AttrStr::new("__origin__");
    static ref ATTR_ENUM_METADATA: AttrStr = AttrStr::new("_perde_metadata");
    static ref ATTR_TYPENAME: AttrStr = AttrStr::new("__name__");
    static ref ATTR_DICT: AttrStr = AttrStr::new("__dict__");
}

pub fn resolve_schema<'a>(
    p: &'a ObjectRef,
    attr: Option<HashMap<&str, &ObjectRef>>,
) -> Result<Cow<'a, Schema>> {
    if p.is_bool() {
        Ok(static_schema().boolean.borrowed())
    } else if p.is_str() {
        Ok(static_schema().string.borrowed())
    } else if p.is_int() {
        Ok(static_schema().int.borrowed())
    } else if p.is_float() {
        Ok(static_schema().float.borrowed())
    } else if p.is_bytes() {
        Ok(static_schema().bytes.borrowed())
    } else if p.is_bytearray() {
        Ok(static_schema().bytearray.borrowed())
    } else if p.is_dict() {
        Ok(static_schema().dict.borrowed())
    } else if p.is_list() {
        Ok(static_schema().list.borrowed())
    } else if p.is_set() {
        Ok(static_schema().set.borrowed())
    } else if p.is_frozen_set() {
        Ok(static_schema().frozenset.borrowed())
    } else if p.is_tuple() {
        Ok(static_schema().tuple.borrowed())
    } else if p.is_none_type() || p.is_any() {
        Ok(static_schema().any.borrowed())
    } else if p.is_datetime() {
        Ok(static_schema().datetime.borrowed())
    } else if p.is_time() {
        Ok(static_schema().time.borrowed())
    } else if p.is_date() {
        Ok(static_schema().date.borrowed())
    } else if p.is_decimal() {
        Ok(static_schema().decimal.borrowed())
    } else if p.is_uuid() {
        Ok(static_schema().uuid.borrowed())
    } else if p.is_builtin_generic() {
        to_generic(p).map(|s| s.owned())
    } else {
        if let Some(p) = p.get_capsule::<Schema>(&SCHEMA_CACHE) {
            return Ok(p.borrowed());
        }

        let s = if p.has_attr(&DATACLASS_FIELDS) {
            to_dataclass(p, &attr)?
        } else if p.is_generic() {
            to_generic(p)?
        } else if p.is_enum() {
            to_enum(p, &attr)?
        } else if !p.is_type() {
            bail_type_err!("`{:?}` is not a type", p)
        } else {
            bail_type_err!("unsupported type `{:?}`", p)
        };

        p.set_capsule::<Schema>(&SCHEMA_CACHE, s)
            .map(|s| s.borrowed())
    }
}

fn to_schema(p: &ObjectRef) -> Result<Schema> {
    resolve_schema(p, None).map(|s| s.into_owned())
}

fn to_dataclass(p: &ObjectRef, attr: &Option<HashMap<&str, &ObjectRef>>) -> Result<Schema> {
    let cattr = ClassAttr::parse(attr)?;

    let fields = import()?.fields.call1(p.owned())?;
    let fields = fields.get_tuple_iter()?;

    let mut members = IndexMap::new();
    let mut skip_field_len = 0;
    let mut flatten_dict = None;

    let missing = &import()?.missing;

    for (i, field) in fields.enumerate() {
        let name = field.get_attr(&ATTR_NAME)?;
        let ty = field.get_attr(&ATTR_TYPE)?;
        let default = field
            .get_attr(&ATTR_DEFAULT)?
            .into_opt()
            .filter(|o| !o.is(missing.as_ptr()));
        let default_factory = field
            .get_attr(&ATTR_DEFAULT_FACTORY)?
            .into_opt()
            .filter(|o| !o.is(missing.as_ptr()));
        let metadata = field.get_attr(&ATTR_METADATA)?.into_opt();

        let fattr = FieldAttr::parse(metadata, default, default_factory)?;

        let origname = name.as_str()?;
        let (dename, sename) = if let Some(renamed) = &fattr.rename {
            (renamed.to_owned(), renamed.to_owned())
        } else if cattr.rename_all.is_some() {
            let renamed = convert_stringcase(origname, cattr.rename_all);
            (renamed.clone(), renamed)
        } else {
            (
                convert_stringcase(origname, cattr.rename_all_deserialize),
                convert_stringcase(origname, cattr.rename_all_serialize),
            )
        };

        if (fattr.skip || fattr.skip_serializing) && !fattr.flatten {
            skip_field_len += 1;
        }

        let schema = to_schema(ty.as_ref())?;

        // Setup flatten dict which absorbs all the remaining fields.
        if fattr.flatten {
            if let Schema::Dict(d) = &schema {
                if flatten_dict.is_none() {
                    flatten_dict = Some(d.clone());
                }
            }
        }

        // `sename` is used for serialization.
        let mem = FieldSchema::new(AttrStr::new(origname), sename, i as usize, fattr, schema);

        // `dename` is for look up schema on deserialization.
        members.insert(dename, mem);
    }

    let name = p.name();
    let class = p.owned();
    let (flatten_members, flatten_skip_len) = collect_flatten_members(&members);

    let ser_field_len = if flatten_members.is_empty() {
        members.len() - skip_field_len
    } else {
        flatten_members.len() - flatten_skip_len
    };

    Ok(Schema::Class(Class::new(
        class.into(),
        name.into(),
        cattr,
        members,
        flatten_members,
        flatten_dict,
        ser_field_len,
    )))
}

fn to_enum(p: &ObjectRef, attr: &Option<HashMap<&str, &ObjectRef>>) -> Result<Schema> {
    let eattr = EnumAttr::parse(&attr)?;

    let iter = p.get_iter()?;

    let variants: Result<_> = iter
        .map(|item| {
            let item = item?;
            let name = item.get_attr(&ATTR_NAME)?;
            let value = item.get_attr(&ATTR_VALUE)?;

            let attr = if item.has_attr(&ATTR_ENUM_METADATA) {
                let metadata = item.get_attr(&ATTR_ENUM_METADATA)?;
                VariantAttr::parse(&Some(&metadata))?
            } else {
                VariantAttr::default()
            };

            let origname = name.as_str()?;

            let (dename, sername) = if let Some(renamed) = attr.rename.as_ref() {
                (renamed.to_string(), renamed.to_string())
            } else if eattr.rename_all.is_some() {
                let renamed = convert_stringcase(origname, eattr.rename_all);
                (renamed.clone(), renamed)
            } else {
                (
                    convert_stringcase(origname, eattr.rename_all_deserialize),
                    convert_stringcase(origname, eattr.rename_all_serialize),
                )
            };

            Ok(VariantSchema::new(
                origname.into(),
                sername,
                dename,
                attr,
                value.into(),
            ))
        })
        .collect();

    Ok(Schema::Enum(Enum::new(
        p.name().into(),
        p.owned().into(),
        eattr,
        variants?,
    )))
}

fn to_union(args: &ObjectRef) -> Result<Schema> {
    let args = args.get_tuple_iter()?;

    let mut optional = false;
    let variants: Result<Vec<_>> = args
        .filter_map(|arg| {
            if arg.is_none_type() {
                optional = true;
                None
            } else {
                Some(to_schema(arg))
            }
        })
        .collect();

    Ok(Schema::Union(Union::new(variants?, optional)))
}

fn to_tuple(args: &ObjectRef) -> Result<Schema> {
    let mut args = args.get_tuple_iter()?;

    if args.len() == 1 {
        let p = args
            .next()
            .ok_or_else(|| type_err!("cannot get element type"))?;
        if p.is(import()?.empty_tuple.as_ptr()) {
            return Ok(Schema::Tuple(Tuple::new(vec![])));
        }
    }

    let args: Result<_> = args.map(|arg| to_schema(arg)).collect();
    let args: Vec<_> = args?;
    if args.is_empty() {
        // Here is for Tuple without subscription.
        // `typing.Tuple[]` is syntax error.
        // i.e. empty args always means typing.Tuple.
        // It accepts any types.
        return Ok(Schema::Tuple(Tuple::any_tuple()));
    }
    Ok(Schema::Tuple(Tuple::new(args)))
}

fn to_dict(args: &ObjectRef) -> Result<Schema> {
    let mut args = args.get_tuple_iter()?;
    let key = arg_to_schema(&mut args).context("invalid key type in `dict`")?;
    let value = arg_to_schema(&mut args).context("invalid value type in `dict`")?;
    Ok(Schema::Dict(Dict::new(Box::new(key), Box::new(value))))
}

fn to_list(args: &ObjectRef) -> Result<Schema> {
    let mut args = args.get_tuple_iter()?;
    let value = arg_to_schema(&mut args).context("invalid element type in `list`")?;
    Ok(Schema::List(List::new(Box::new(value))))
}

fn to_set(args: &ObjectRef) -> Result<Schema> {
    let mut args = args.get_tuple_iter()?;
    let value = arg_to_schema(&mut args).context("invalid element type in `set`")?;
    Ok(Schema::Set(Set::new(Box::new(value))))
}

fn to_frozen_set(args: &ObjectRef) -> Result<Schema> {
    let mut args = args.get_tuple_iter()?;
    let value = arg_to_schema(&mut args).context("invalid element type in `frozenset`")?;
    Ok(Schema::FrozenSet(FrozenSet::new(Box::new(value))))
}

fn to_generic(p: &ObjectRef) -> Result<Schema> {
    let origin = p.get_attr(&ATTR_ORIGIN)?;
    let args = p.get_attr(&ATTR_ARGS)?;

    let s = if origin.is(import()?.union.as_ptr()) {
        to_union(&args)
    } else if origin.is_tuple() {
        to_tuple(&args)
    } else if origin.is_dict() {
        to_dict(&args)
    } else if origin.is_set() {
        to_set(&args)
    } else if origin.is_list() {
        to_list(&args)
    } else if origin.is_frozen_set() {
        to_frozen_set(&args)
    } else {
        bail_type_err!("unsupported generic type: {:?}", p);
    };

    s.context(format!("cannot get generic type information: `{:?}`", p))
}

fn arg_to_schema(args: &mut TupleIter) -> Result<Schema> {
    Ok(args
        .next()
        .map(|arg| to_schema(arg))
        .transpose()?
        .unwrap_or_else(|| static_schema().any.clone()))
}
