use crate::data::{random_field_name, random_type_name};
use derive_new::new;
use indexmap::IndexMap;
use rand::{
    distributions::{Alphanumeric, Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

thread_local! {
    pub static DEPTH: AtomicUsize = AtomicUsize::new(0);
    pub static INIT: AtomicBool = AtomicBool::new(false);
}

fn set_depth(depth: usize) {
    DEPTH.with(|d| {
        d.store(depth, Ordering::Relaxed);
    });
}

fn go_deep() -> bool {
    DEPTH.with(|d| {
        let depth = d.load(Ordering::Relaxed);
        if depth == 0 {
            return false;
        }
        d.fetch_sub(1, Ordering::Relaxed);
        true
    })
}

fn go_up() {
    DEPTH.with(|d| d.fetch_add(1, Ordering::Relaxed));
}

fn set_init() {
    INIT.with(|b| b.swap(true, Ordering::Relaxed));
}

fn is_init() -> bool {
    INIT.with(|b| b.swap(false, Ordering::Relaxed))
}

macro_rules! opt {
    ($rng:expr, $v:expr) => {
        if $rng.gen() {
            Some($v)
        } else {
            None
        }
    };
}

#[derive(Clone, Debug, Default, PartialEq, Eq, new, Serialize, Deserialize)]
pub struct FieldAttr {
    pub flatten: bool,
    pub rename: Option<String>,
    pub default: bool,
    pub skip: bool,
    pub skip_deserializing: bool,
}

impl Distribution<FieldAttr> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FieldAttr {
        FieldAttr::new(
            rng.gen(),
            opt!(rng, random_field_name(rng)),
            rng.gen(),
            rng.gen(),
            rng.gen(),
        )
    }
}

#[derive(Clone, Debug, Default, new, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariantAttr {
    pub rename: Option<String>,
}

impl Distribution<VariantAttr> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> VariantAttr {
        VariantAttr::new(opt!(rng, random_type_name(rng)))
    }
}

#[derive(Clone, Debug, Default, new, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassAttr {
    pub rename_all: Option<String>,
    pub rename: Option<String>,
    pub deny_unknown_fields: bool,
    pub default: bool,
}

impl Distribution<ClassAttr> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ClassAttr {
        let v: usize = rng.gen_range(0, 9);
        let attr = match v {
            0 => Some("lowercase".into()),
            1 => Some("UPPERCASE".into()),
            2 => Some("PascalCase".into()),
            3 => Some("camelCase".into()),
            4 => Some("snake_case".into()),
            5 => Some("SCREAMING_SNAKE_CASE".into()),
            6 => Some("kebab-case".into()),
            7 => Some("SCREAMING-KEBAB-CASE".into()),
            _ => None,
        };

        ClassAttr::new(attr, opt!(rng, random_type_name(rng)), rng.gen(), rng.gen())
    }
}

#[derive(Clone, Debug, Default, new, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnumAttr {
    pub rename_all: Option<String>,
    pub rename: Option<String>,
}

impl Distribution<EnumAttr> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnumAttr {
        EnumAttr::new(
            opt!(rng, random_field_name(rng)),
            opt!(rng, random_field_name(rng)),
        )
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dict {
    pub key: Box<Schema>,
    pub value: Box<Schema>,
}

impl Distribution<Dict> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dict {
        Dict::new(Box::new(Schema::Str), Box::new(rng.gen()))
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq, Serialize, Deserialize)]
pub struct List {
    pub value: Box<Schema>,
}

impl Distribution<List> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> List {
        List::new(Box::new(rng.gen()))
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq, Serialize, Deserialize)]
pub struct Set {
    pub value: Box<Schema>,
}

impl Distribution<Set> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Set {
        let set = loop {
            let s = rng.gen();
            match s {
                Schema::Float => continue,
                Schema::List(_) => continue,
                Schema::Class(_) => continue,
                Schema::Tuple(_) => continue,
                Schema::Optional(_) => continue,
                Schema::Dict(_) => continue,
                Schema::Set(_) => continue,
                _ => break s,
            }
        };
        Set::new(Box::new(set))
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tuple {
    pub args: Vec<Schema>,
}

impl Distribution<Tuple> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tuple {
        let v: usize = rng.gen_range(0, 3);
        Tuple::new((0..v).map(|_| rng.gen()).collect())
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub attr: EnumAttr,
    pub variants: Vec<String>,
}

impl Distribution<Enum> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Enum {
        let v: usize = rng.gen_range(0, 3);
        Enum::new(
            random_type_name(rng),
            rng.gen(),
            (0..v).map(|_| random_type_name(rng)).collect(),
        )
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub attr: ClassAttr,
    pub fields: IndexMap<String, FieldSchema>,
}

impl Distribution<Class> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Class {
        let v: usize = rng.gen_range(0, 3);

        let class_attr: ClassAttr = rng.gen();

        Class::new(
            random_type_name(rng),
            class_attr,
            (0..v)
                .map(|_| (random_field_name(rng), rng.gen::<FieldSchema>()))
                .collect(),
        )
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldSchema {
    pub attr: FieldAttr,
    pub schema: Schema,
}

impl Distribution<FieldSchema> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FieldSchema {
        FieldSchema::new(rng.gen(), rng.gen())
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq, Serialize, Deserialize)]
pub struct Optional {
    pub value: Box<Schema>,
}

impl Distribution<Optional> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Optional {
        Optional::new(Box::new(rng.gen()))
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq, Serialize, Deserialize)]
pub struct Union {
    pub name: String,
    pub variants: Vec<Schema>,
}

impl Distribution<Union> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Union {
        Union::new(random_type_name(rng), gen_unique_schema(5, rng))
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq, Serialize, Deserialize)]
pub enum Schema {
    Bool,
    Int,
    Float,
    Str,
    Bytes,
    Dict(Dict),
    List(List),
    Set(Set),
    Tuple(Tuple),
    Class(Class),
    Enum(Enum),
    Optional(Optional),
    Union(Union),
}

impl Schema {
    fn constraint(&mut self) {
        let s = self.clone();

        match self {
            Self::Dict(d) => {
                d.key.constraint();
                d.value.constraint();
            }
            Self::List(l) => {
                l.value.constraint();
            }
            Self::Set(s) => {
                s.value.constraint();
            }
            Self::Tuple(t) => t.args.iter_mut().for_each(|v| v.constraint()),
            Self::Class(c) => {
                if !s.has_default() {
                    c.attr.default = false;
                }
                let mut num = c.fields.len();
                c.fields.iter_mut().for_each(|(_, v)| {
                    let can_flatten = match &v.schema {
                        Schema::Dict(_) => true,
                        Schema::Class(_) => true,
                        _ => false,
                    };

                    if !can_flatten {
                        v.attr.flatten = false;
                    }
                    if !v.schema.has_default() {
                        v.attr.default = false;
                        v.attr.skip = false;
                        v.attr.skip_deserializing = false;
                    }
                    if v.attr.flatten {
                        v.attr.rename = None;
                    }
                    if v.attr.skip {
                        v.attr.flatten = false;
                        v.attr.rename = None;
                        v.attr.default = false;
                        v.attr.skip_deserializing = false;
                    }
                    if num == 1 {
                        v.attr.skip = false;
                    }

                    if v.attr.skip || v.attr.skip_deserializing || v.attr.flatten {
                        num -= 1;
                    }

                    v.schema.constraint()
                });

                if num == 0 {
                    c.attr.deny_unknown_fields = false;
                }
            }
            Self::Optional(o) => o.value.constraint(),
            Self::Union(u) => u.variants.iter_mut().for_each(|u| u.constraint()),
            _ => {}
        }
    }

    fn valid(&self) -> bool {
        match self {
            Self::Bool => true,
            Self::Int => true,
            Self::Float => true,
            Self::Str => true,
            Self::Bytes => true,
            Self::Dict(d) => d.key.valid() && d.key.has_hash() && d.value.valid(),
            Self::List(l) => l.value.valid(),
            Self::Set(s) => s.value.valid() && s.value.has_hash(),
            Self::Tuple(t) => t.args.iter().all(|v| v.valid()),
            Self::Class(c) => c.fields.iter().all(|(_, v)| v.schema.valid()),
            Self::Enum(e) => true,
            Self::Optional(o) => o.value.valid(),
            Self::Union(u) => u.variants.iter().all(|u| u.valid()),
        }
    }

    pub fn has_default(&self) -> bool {
        match self {
            Self::Bool => true,
            Self::Int => true,
            Self::Float => true,
            Self::Str => true,
            Self::Bytes => true,
            Self::Dict(d) => true,
            Self::List(l) => true,
            Self::Set(s) => true,
            Self::Tuple(t) => t.args.iter().all(|v| v.has_default()),
            Self::Class(c) => c.fields.iter().all(|(_, v)| v.schema.has_default()),
            Self::Enum(e) => false,
            Self::Optional(o) => o.value.has_default(),
            Self::Union(u) => false,
        }
    }

    pub fn has_hash(&self) -> bool {
        match self {
            Self::Bool => true,
            Self::Int => true,
            Self::Float => false,
            Self::Str => true,
            Self::Bytes => true,
            Self::Dict(d) => false,
            Self::List(l) => l.value.has_hash(),
            Self::Set(s) => false,
            Self::Tuple(t) => t.args.iter().all(|v| v.has_hash()),
            Self::Class(c) => c.fields.iter().all(|(_, v)| v.schema.has_hash()),
            Self::Enum(e) => true,
            Self::Optional(o) => o.value.has_hash(),
            Self::Union(u) => u.variants.iter().all(|u| u.has_hash()),
        }
    }

    pub fn has_eq(&self) -> bool {
        match self {
            Self::Bool => true,
            Self::Int => true,
            Self::Float => false,
            Self::Str => true,
            Self::Bytes => true,
            Self::Dict(d) => d.key.has_eq() && d.value.has_eq(),
            Self::List(l) => l.value.has_eq(),
            Self::Set(s) => s.value.has_eq(),
            Self::Tuple(t) => t.args.iter().all(|v| v.has_eq()),
            Self::Class(c) => c.fields.iter().all(|(_, v)| v.schema.has_eq()),
            Self::Enum(e) => true,
            Self::Optional(o) => o.value.has_eq(),
            Self::Union(u) => u.variants.iter().all(|u| u.has_eq()),
        }
    }

    fn is_map(&self) -> bool {
        match self {
            Self::Dict(_) => true,
            _ => false,
        }
    }

    fn is_vec(&self) -> bool {
        match self {
            Self::Bytes => true,
            Self::List(_) => true,
            _ => false,
        }
    }

    fn is_set(&self) -> bool {
        match self {
            Self::Set(_) => true,
            _ => false,
        }
    }

    fn is_opt(&self) -> bool {
        match self {
            Self::Optional(_) => true,
            _ => false,
        }
    }
}

fn num_to_random_schema<R: Rng + ?Sized>(num: usize, rng: &mut R) -> Schema {
    match num {
        0 => Schema::Bool,
        1 => Schema::Int,
        2 => Schema::Float,
        3 => Schema::Str,
        4 => Schema::Bytes,
        5 => Schema::Dict(rng.gen()),
        6 => Schema::List(rng.gen()),
        7 => Schema::Set(rng.gen()),
        8 => Schema::Tuple(rng.gen()),
        9 | 10 => Schema::Class(rng.gen()),
        // 10 => Schema::Enum(rng.gen()),
        11 => Schema::Optional(rng.gen()),
        12 => Schema::Union(rng.gen()),
        _ => unreachable!(),
    }
}

fn gen_unique_schema<R: Rng + ?Sized>(count: usize, rng: &mut R) -> Vec<Schema> {
    let m = if go_deep() { 13 } else { 5 };
    let mut nums: Vec<_> = (1..count).map(|_| rng.gen_range(0, m)).collect();
    nums.sort();
    nums.dedup();
    let s = nums
        .into_iter()
        .map(|n| num_to_random_schema(n, rng))
        .collect();
    go_up();
    s
}

impl Distribution<Schema> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Schema {
        let m = if go_deep() { 13 } else { 5 };
        let s = loop {
            let v: usize = rng.gen_range(0, m);
            let s = num_to_random_schema(v, rng);
            if s.valid() {
                let mut s = s;
                s.constraint();
                break s;
            }
        };
        go_up();
        s
    }
}

pub fn gen_schema(depth: usize) -> Schema {
    let mut rng = rand::thread_rng();
    set_init();
    set_depth(depth);
    rng.gen()
}
