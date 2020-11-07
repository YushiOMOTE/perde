use derive_new::new;
use indexmap::IndexMap;
use rand::{
    distributions::{Alphanumeric, Distribution, Standard},
    Rng,
};
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

fn random_type_name<R: Rng + ?Sized>(rng: &mut R) -> String {
    let tail: String = rng.sample_iter(&Alphanumeric).take(5).collect();
    format!("T{}", tail)
}

fn random_field_name<R: Rng + ?Sized>(rng: &mut R) -> String {
    let tail: String = rng.sample_iter(&Alphanumeric).take(5).collect();
    format!("f{}", tail.to_lowercase())
}

#[derive(Clone, Debug, Default, PartialEq, Eq, new)]
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

#[derive(Clone, Debug, Default, new, PartialEq, Eq)]
pub struct VariantAttr {
    pub rename: Option<String>,
}

impl Distribution<VariantAttr> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> VariantAttr {
        VariantAttr::new(opt!(rng, random_type_name(rng)))
    }
}

#[derive(Clone, Debug, Default, new, PartialEq, Eq)]
pub struct ClassAttr {
    pub rename_all: Option<String>,
    pub rename: Option<String>,
    pub deny_unknown_fields: bool,
    pub default: bool,
}

impl Distribution<ClassAttr> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ClassAttr {
        ClassAttr::new(
            opt!(rng, random_type_name(rng)),
            opt!(rng, random_type_name(rng)),
            rng.gen(),
            rng.gen(),
        )
    }
}

#[derive(Clone, Debug, Default, new, PartialEq, Eq)]
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

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct Dict {
    pub key: Box<Schema>,
    pub value: Box<Schema>,
}

impl Distribution<Dict> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Dict {
        Dict::new(Box::new(rng.gen()), Box::new(rng.gen()))
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct List {
    pub value: Box<Schema>,
}

impl Distribution<List> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> List {
        List::new(Box::new(rng.gen()))
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct Set {
    pub value: Box<Schema>,
}

impl Distribution<Set> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Set {
        Set::new(Box::new(rng.gen()))
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct Tuple {
    pub args: Vec<Schema>,
}

impl Distribution<Tuple> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tuple {
        let v: usize = rng.gen_range(0, 3);
        Tuple::new((0..v).map(|_| rng.gen()).collect())
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
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

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct Class {
    pub name: String,
    pub attr: ClassAttr,
    pub fields: IndexMap<String, FieldSchema>,
}

impl Distribution<Class> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Class {
        let v: usize = rng.gen_range(0, 3);

        Class::new(
            random_type_name(rng),
            rng.gen(),
            (0..v)
                .map(|_| (random_field_name(rng), rng.gen()))
                .collect(),
        )
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct FieldSchema {
    pub attr: FieldAttr,
    pub schema: Schema,
}

impl Distribution<FieldSchema> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FieldSchema {
        FieldSchema::new(rng.gen(), rng.gen())
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct Optional {
    pub value: Box<Schema>,
}

impl Distribution<Optional> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Optional {
        Optional::new(Box::new(rng.gen()))
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
pub struct Union {
    pub variants: Vec<Schema>,
}

impl Distribution<Union> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Union {
        let v: usize = rng.gen_range(0, 3);
        Union::new((0..v).map(|_| rng.gen()).collect())
    }
}

#[derive(Debug, Clone, new, PartialEq, Eq)]
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

impl Distribution<Schema> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Schema {
        let m = if go_deep() { 13 } else { 5 };
        let v: usize = rng.gen_range(0, m);
        match v {
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
}

pub fn gen_schema(depth: usize) -> Schema {
    let mut rng = rand::thread_rng();
    set_init();
    set_depth(depth);
    rng.gen()
}
