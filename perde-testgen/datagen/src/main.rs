#[macro_use]
mod defs;
mod gen;

use crate::{
    defs::*,
    gen::{GenExt, Random},
};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    base: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    macro_rules! write {
        ($name:expr, $encoder:path) => {
            let dir = opt.base.join($name);
            fs::create_dir_all(&dir).unwrap();
            for (i, item) in gen!($encoder).into_iter().enumerate() {
                fs::write(dir.join(i.to_string()), item).unwrap();
            }
        };
    }

    write!("json", serde_json::to_vec);
    write!("yaml", serde_yaml::to_vec);
    // write!("msgpack", rmp_serde::to_vec);
}
