#[macro_use]
mod defs;
mod gen;

use crate::{defs::*, gen::GenExt};
use std::{fs, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// The formats to generate.
    #[structopt(short = "f", long = "format")]
    format: Vec<String>,
    /// The base directory of the data set.
    #[structopt(short = "d", long = "dir")]
    base: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();

    let cur_dir = std::env::current_dir().unwrap();

    macro_rules! write {
        ($name:expr, $encoder:path) => {{
            let dir = opt.base.as_ref().unwrap_or_else(|| &cur_dir).join($name);
            fs::create_dir_all(&dir).unwrap();
            for (i, item) in gen!($encoder).into_iter().enumerate() {
                fs::write(dir.join(i.to_string()), item).unwrap();
            }
        }};
    }

    for f in &opt.format {
        match f.as_ref() {
            "json" => write!("json", serde_json::to_vec),
            "yaml" => write!("yaml", serde_yaml::to_vec),
            "msgpack" => write!("msgpack", rmp_serde::to_vec_named),
            f => panic!("unknown format: {}", f),
        }
    }
}
