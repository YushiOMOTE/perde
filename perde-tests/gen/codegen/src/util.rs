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
                            $encoder(&data).expect(&format!(
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
        std::fs::create_dir_all(&d).expect(&format!("cannot create directory: {}", d.display()));
        let d = d.join(name);
        std::fs::write(&d, data).expect(&format!("cannot write: {}", d.display()));
    }
}
