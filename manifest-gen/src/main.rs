use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use structopt::StructOpt;
use tera::{Context, Tera};
use tera_text_filters::snake_case;
use walkdir::WalkDir;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Format {
    crate_name: String,
    version: String,
    description: String,
    #[serde(default)]
    deps: HashMap<String, String>,
    #[serde(flatten)]
    extra: serde_yaml::Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Config {
    generator_note: String,
    pyo3_version: String,
    perde_core_version: String,
    formats_test_version: String,
    formats: Vec<Format>,
}

#[derive(StructOpt)]
struct Opt {
    /// Templates directory.
    #[structopt(short = "T", long = "templates")]
    templates: String,
    /// Temporarily override formats versions for publish testing.
    #[structopt(short = "t", long = "test_version")]
    test_version: bool,
    /// Config file.
    #[structopt(name = "config")]
    config: String,
    /// Output directory.
    #[structopt(name = "output_dir", default_value = ".")]
    output_dir: String,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let cfg: Config = serde_yaml::from_reader(std::fs::File::open(&opt.config)?)?;

    let mut tera = Tera::new(&format!("{}/**", opt.templates))?;

    tera.register_filter("snake_case", snake_case);

    for format in &cfg.formats {
        let mut format = format.clone();
        if opt.test_version {
            format.version = cfg.formats_test_version.clone();
        }

        let mut context = Context::new();
        context.insert("generator_note", &cfg.generator_note);
        context.insert("vars", &cfg);
        context.insert("format", &format);

        println!("Rendering format: {:?}", format);

        let d = WalkDir::new(&opt.templates);

        for e in d
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let name = e.file_name().to_string_lossy();
            let rendered = tera.render(&name, &context)?;
            let path = format!("{}/{}/{}", opt.output_dir, format.crate_name, name);
            println!("Rendering file: {}", path);
            std::fs::write(&path, rendered)?;
        }
    }

    Ok(())
}
