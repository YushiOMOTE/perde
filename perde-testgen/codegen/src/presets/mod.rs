use crate::schema::*;
use indexmap::IndexMap;
use std::sync::atomic::{AtomicUsize, Ordering};

static NAME_INDEX: AtomicUsize = AtomicUsize::new(0);

macro_rules! map {
    ($($k:expr => $v:expr),*) => {{
        let mut map = IndexMap::new();
        $(map.insert($k.into(), $v.into());)*
        map
    }};
}

fn name() -> String {
    format!("Preset{}", NAME_INDEX.fetch_add(1, Ordering::Relaxed))
}

fn preset_rename() -> Vec<Schema> {
    vec![Schema::Class(Class::new(
        name(),
        ClassAttr::default(),
        map!(
            "a" => FieldSchema::new(FieldAttr::builder().rename(Some("hage".into())).build().unwrap(), Schema::Bool),
            "b" => FieldSchema::new(FieldAttr::default(), Schema::Int)
        ),
    ))]
}

pub fn presets(name: &str) -> Vec<Schema> {
    match name {
        "rename" => preset_rename(),
        s => panic!("unknown preset name: {}", s),
    }
}
