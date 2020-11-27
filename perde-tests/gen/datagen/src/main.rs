// This file is generated. Don't modify manually.

use derive_new::new;
use indexmap::IndexMap;
use serde::Serialize;
use std::{
    collections::HashMap,
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Mutex,
};

lazy_static::lazy_static! {
    static ref DATA: Mutex<HashMap<String, HashMap<String, Vec<u8>>>> = Mutex::new(HashMap::new());
}

fn add_value<T: Serialize + Debug>(name: &str, data: T) {
    add_value_except(name, data, &[]);
}

fn add_value_except<T: Serialize + Debug>(name: &str, data: T, excepts: &[&str]) {
    let mut map = DATA.lock().unwrap();

    macro_rules! insert {
        ($format:expr, $encoder:path) => {
            if !excepts.contains(&$format) {
                assert!(
                    map.entry($format.into())
                        .or_default()
                        .insert(
                            name.into(),
                            $encoder(&data).unwrap_or_else(|_| panic!(
                                "couldn't serialize data `{}` in `{}`: {:?}",
                                name, $format, data
                            ))
                        )
                        .is_none(),
                    "`{}` already exists in format `{}`",
                    name,
                    $format
                )
            }
        };
    }

    insert!("json", serde_json::to_vec);
    insert!("yaml", serde_yaml::to_vec);
    insert!("msgpack", rmp_serde::to_vec_named);
    insert!("toml", toml::to_vec);
}

macro_rules! add {
    ($typename:ty {$($value:expr),*}) => {
        add_value(stringify!($typename), <$typename>::new($($value),*));
    };
    ($typename:ty {$($value:expr),*} except $($except:literal),*) => {
        add_value_except(stringify!($typename), <$typename>::new($($value),*), &[$($except),*]);
    }
}

fn save(path: &Path) {
    let map = DATA.lock().unwrap();

    for (fmt, name, data) in map
        .iter()
        .map(|(f, map)| map.iter().map(move |(n, d)| (f.clone(), n, d)))
        .flatten()
    {
        let d = path.to_path_buf().join(fmt);
        std::fs::create_dir_all(&d)
            .unwrap_or_else(|_| panic!("cannot create directory: {}", d.display()));
        let d = d.join(name);
        std::fs::write(&d, data).unwrap_or_else(|_| panic!("cannot write: {}", d.display()));
    }
}

use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// Output directory
    #[structopt(name = "dir")]
    dir: PathBuf,
}

fn main() {
    add_value("BenchNumber", 1311);
    add_value("BenchString", "aiueo");

    add_value("BenchDict", {
        let mut map = IndexMap::new();
        map.insert("10".to_string(), 10000);
        map.insert("101".into(), 100030);
        map.insert("102".into(), 102000);
        map
    });

    add_value("BenchList", vec![1i64, 2, -3, 4, 5, -8]);

    #[derive(Serialize, Debug, new)]
    struct Plain {
        a: String,
        b: String,
        c: u64,
    }

    add!(Plain {"xxx".into(), "yyy".into(), 3});

    #[derive(Serialize, Debug, new)]
    #[serde(rename_all = "camelCase")]
    struct RenameAll {
        pen_pineapple: String,
        apple_pen: String,
    }

    add!(RenameAll {"xxx".into(), "yyy".into()});

    #[derive(Serialize, Debug, new)]
    #[serde(rename = "RenameAllSerialize", rename_all = "PascalCase")]
    struct RenameAllSerializeOutput {
        pen_pineapple: String,
        apple_pen: String,
    }

    #[derive(Serialize, Debug, new)]
    #[serde(rename = "RenameAllSerialize")]
    struct RenameAllSerializeInput {
        pen_pineapple: String,
        apple_pen: String,
    }

    add!(RenameAllSerializeInput {"--".into(), "==".into()});
    add!(RenameAllSerializeOutput {"--".into(), "==".into()});

    #[derive(Serialize, Debug, new)]
    #[serde(rename = "RenameAllDeserialize")]
    struct RenameAllDeserializeOutput {
        pen_pineapple: String,
        apple_pen: String,
    }

    #[derive(Serialize, Debug, new)]
    #[serde(rename = "RenameAllDeserialize", rename_all = "SCREAMING_SNAKE_CASE")]
    struct RenameAllDeserializeInput {
        pen_pineapple: String,
        apple_pen: String,
    }

    add!(RenameAllDeserializeInput {"--".into(), "==".into()});
    add!(RenameAllDeserializeOutput {"--".into(), "==".into()});

    #[derive(Serialize, Debug, new)]
    struct DenyUnknownFields {
        x: String,
        y: i64,
        z: i64,
        q: String,
    }

    add!(DenyUnknownFields {"aaaaa".into(), 1, -2, "unknown".into()});

    #[derive(Serialize, Debug, new)]
    struct Rename {
        a: String,
        #[serde(rename = "x")]
        b: String,
        c: u64,
    }

    add!(Rename {"xxx".into(), "yyy".into(), 3});

    #[derive(Serialize, Debug, new)]
    #[serde(rename_all = "camelCase")]
    struct RenameAllRename {
        pen_pineapple: String,
        #[serde(rename = "pen_pen")]
        apple_pen: String,
    }

    add!(RenameAllRename {"xxx".into(), "yyy".into()});

    #[derive(Serialize, Debug, new)]
    struct NestedRenameChild {
        a: String,
        #[serde(rename = "d")]
        b: String,
    }

    #[derive(Serialize, Debug, new)]
    struct NestedRename {
        x: String,
        #[serde(rename = "w")]
        y: NestedRenameChild,
        z: i64,
    }

    add!(NestedRename
     {"xxx".into(),
      NestedRenameChild::new("ppp".into(), "qqq".into()),
      1111}
     except "toml");

    #[derive(Serialize, Debug, new)]
    #[serde(rename_all = "UPPERCASE")]
    struct NestedRenameAllChild {
        a: String,
        b: String,
    }

    #[derive(Serialize, Debug, new)]
    struct NestedRenameAll {
        x: String,
        y: NestedRenameAllChild,
        z: i64,
    }

    add!(NestedRenameAll
     {"xxx".into(),
      NestedRenameAllChild::new("ppp".into(), "qqq".into()),
      1111}
     except "toml");

    #[derive(Serialize, Debug, new)]
    struct FlattenChild {
        a: String,
        b: String,
    }

    #[derive(Serialize, Debug, new)]
    struct Flatten {
        x: String,
        #[serde(flatten)]
        y: FlattenChild,
        z: i64,
    }

    add!(Flatten
     {"xxx".into(),
      FlattenChild::new("ppp".into(), "qqq".into()),
      1111}
     except "msgpack");

    #[derive(Serialize, Debug, new)]
    struct DictFlatten {
        x: String,
        y: i64,
        #[serde(flatten)]
        z: IndexMap<String, String>,
    }

    add!(DictFlatten {"hey".into(), -103223,
    {
     let mut m = IndexMap::new();
     m.insert("pp".into(), "q1".into());
     m.insert("ppp".into(), "q2".into());
     m.insert("pppp".into(), "q3".into());
     m
    }}
     except "msgpack");

    #[derive(Serialize, Debug, new)]
    struct Flatten2 {
        x: String,
        a: i64,
        b: i64,
    }

    add!(Flatten2 { "haa".into(), 11, 33 });

    #[derive(Serialize, Debug, new)]
    struct DictFlatten2 {
        x: String,
        y: i64,
        pp: String,
        ppp: String,
        pppp: String,
    }

    add!(DictFlatten2 {
     "hey".into(), -103223,
     "q1".into(), "q2".into(), "q3".into()
    });

    #[derive(Serialize, Debug, new)]
    struct TypeMismatch {
        a: String,
        b: Vec<u32>,
    }

    add!(TypeMismatch { "hage".into(), vec![1,2,3] });

    #[derive(Serialize, Debug, new)]
    struct MissingMember {
        a: String,
    }

    add!(MissingMember { "hage".into() });

    #[derive(Serialize, Debug, new)]
    struct TooManyMember {
        a: String,
        b: String,
        c: i64,
    }

    add!(TooManyMember { "hage".into(), "faa".into(), 33 });

    #[derive(Serialize, Debug, new)]
    struct SkipEnumError {
        x: i64,
        e: String,
    }

    add!(SkipEnumError { 3, "A".into() });

    #[derive(Serialize, Debug, new)]
    struct DictFlattenMsgpack {
        x: String,
        y: i64,
        pp: String,
        ppp: String,
        pppp: String,
    }

    add!(DictFlattenMsgpack {
     "hey".into(), -103223,
     "q1".into(), "q2".into(), "q3".into()
    });

    #[derive(Serialize, Debug, new)]
    struct DefaultConstruct {
        a: String,
        c: u64,
    }

    add!(DefaultConstruct {"xxx".into(), 3});

    #[derive(Serialize, Debug, new)]
    struct Skip {
        x: String,
        y: u64,
        z: f64,
        a: String,
        b: String,
    }

    #[derive(Serialize, Debug, new)]
    struct Skipped {
        x: String,
        #[serde(skip)]
        y: u64,
        z: f64,
        a: String,
        b: String,
    }

    #[derive(Serialize, Debug, new)]
    struct SkipDefault {
        x: String,
        y: u64,
        z: f64,
        a: String,
        b: String,
    }

    add!(Skip {"ssssss".into(), 3, 1.1, "a".into(), "b".into()});
    add!(Skipped {"ssssss".into(), 3, 1.1, "a".into(), "b".into()});
    add!(SkipDefault {"ssssss".into(), 0, 1.1, "a".into(), "b".into()});

    add_value("EnumX", "X");
    add_value("EnumY", "Y");
    add_value("EnumZ", "Z");

    add_value("EnumXValue", "hi");
    add_value("EnumYValue", "foo");
    add_value("EnumZValue", 3);

    add_value("EnumYRename", "Yay");

    add_value("EnumRenameAllX", "pan-piano");
    add_value("EnumRenameAllY", "pan-piano-good");
    add_value("EnumRenameAllZ", "pan-piano-excellent");

    add_value("EnumRenameAllYRename", "PaiPai");

    add_value("EnumRenameAllXRaw", "PanPiano");
    add_value("EnumRenameAllYRaw", "PanPianoGood");
    add_value("EnumRenameAllZRaw", "PanPianoExcellent");

    add_value("Other", "fafafafa");

    let opt = Opt::from_args();
    save(&opt.dir);
}
