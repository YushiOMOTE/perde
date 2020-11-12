use crate::schema::*;
use indexmap::IndexMap;
use std::sync::atomic::{AtomicUsize, Ordering};

static NAME_INDEX: AtomicUsize = AtomicUsize::new(0);

macro_rules! builder {
    ($attr:ty {$($key:ident => $value:expr),*})  => {
        <$attr>::builder()
            $(.$key($value.into()))*
            .build()
            .unwrap()
    }
}

macro_rules! class_attr {
    ($($key:ident => $value:expr),*)    => {
        builder!(ClassAttr { $($key => $value),* })
    }
}

macro_rules! fields {
    ($($k:expr => $v:expr $(,$ak:ident = $av:expr)* ;)*) => {{
        let mut map = IndexMap::new();
        $(map.insert($k.into(), FieldSchema::new(builder!(FieldAttr { $($ak => $av),* }), $v.into()));)*
        map
    }};
}

fn name() -> String {
    format!("Preset{}", NAME_INDEX.fetch_add(1, Ordering::Relaxed))
}

fn preset_rename() -> Vec<Schema> {
    vec![
        Schema::Class(Class::new(
            name(),
            class_attr!(rename_all => Some("camelCase".into())),
            fields!(
                "apple_pen" => Schema::Bool;
                "pen_pineapple" => Schema::Int;
            ),
        )),
        Schema::Class(Class::new(
            name(),
            ClassAttr::default(),
            fields!(
                "a" => Schema::Bool, rename = Some("hage".into());
                "b" => Schema::Int;
            ),
        )),
        Schema::Class(Class::new(
            name(),
            ClassAttr::default(),
            fields!(
                "a" => Schema::Class(
                    Class::new(name(), class_attr!(), fields!(
                        "x" => Schema::Bool;
                        "y" => Schema::Int;
                    ))
                ), flatten = true;
                "b" => Schema::Int;
            ),
        )),
    ]
}

pub fn presets(name: &str) -> Vec<Schema> {
    match name {
        "rename" => preset_rename(),
        s => panic!("unknown preset name: {}", s),
    }
}
