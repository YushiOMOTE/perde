use crate::schema::*;
use indexmap::IndexMap;

macro_rules! map {
    ($($k:expr => $v:expr),*) => {{
        let mut map = IndexMap::new();
        $(map.insert($k.into(), $v.into());)*
        map
    }};
}

fn preset_rename() -> Vec<Schema> {
    vec![Schema::Class(Class::new(
        "A".into(),
        ClassAttr::default(),
        map!(
            "a" => FieldSchema::new(FieldAttr::default(), Schema::Bool),
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
