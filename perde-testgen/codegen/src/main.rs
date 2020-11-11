use codegen::{gen, gen_schema, Code, Python, Rust, Schema};
use std::{
    fs,
    io::prelude::*,
    path::PathBuf,
    process::{Command, Stdio},
};
use structopt::StructOpt;

pub fn gen_schema_set(num: usize, depth: usize) -> Vec<Schema> {
    (0..num).map(|_| gen_schema(depth)).collect()
}

pub fn gen_code(s: &[Schema]) -> Vec<(Code, Code)> {
    s.iter().map(|s| (gen(Rust, s), gen(Python, s))).collect()
}

fn rustfmt(value: &str) -> String {
    let mut process = Command::new("rustfmt")
        .arg("--emit=stdout")
        .arg("--edition=2018")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = process.stdin.as_mut().unwrap();
    stdin.write_all(value.as_bytes()).unwrap();

    let output = process.wait_with_output().unwrap();
    assert!(output.status.success());

    std::str::from_utf8(&output.stdout).unwrap().to_owned()
}

fn yapf(value: &str) -> String {
    let mut process = Command::new("yapf")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let stdin = process.stdin.as_mut().unwrap();
    stdin.write_all(value.as_bytes()).unwrap();

    let output = process.wait_with_output().unwrap();
    assert!(output.status.success());

    std::str::from_utf8(&output.stdout).unwrap().to_owned()
}

#[derive(StructOpt)]
struct GenSchema {
    /// The number of schema.
    #[structopt(short = "n", long = "number")]
    num: usize,
    /// The size of each schema.
    #[structopt(short = "d", long = "depth")]
    depth: usize,
    /// Path to schema file.
    #[structopt(name = "schema")]
    schema: PathBuf,
}

#[derive(StructOpt)]
struct GenCode {
    /// Path to schema file.
    #[structopt(name = "schema")]
    schema: PathBuf,
    /// Path to the output Rust file.
    #[structopt(name = "rust")]
    rust_file: PathBuf,
    /// Path to the output Python file.
    #[structopt(name = "python")]
    python_file: PathBuf,
}

#[derive(StructOpt)]
enum Opt {
    /// Generate schema
    #[structopt(name = "schema")]
    GenSchema(GenSchema),
    /// Generate code
    #[structopt(name = "code")]
    GenCode(GenCode),
}

fn gen_schema_cmd(c: GenSchema) {
    let schemas = gen_schema_set(c.num, c.depth);
    let schemas = serde_yaml::to_vec(&schemas).unwrap();
    fs::write(&c.schema, &schemas).unwrap();
}

fn gen_code_cmd(c: GenCode) {
    let schemas: Vec<Schema> = serde_yaml::from_slice(&fs::read(&c.schema).unwrap()).unwrap();

    let mut rs_constructs = vec![];
    let mut rs_definitions = vec![];
    let mut py_definitions = vec![];
    let mut py_types = vec![];

    for (rs_code, py_code) in gen_code(&schemas) {
        py_definitions.push(py_code.definitions);
        py_types.push(format!("{}", py_code.typename));
        rs_definitions.push(rs_code.definitions);
        rs_constructs.push(format!(
            r#"
  {construct}
  ret.push($encoder(&v).unwrap());
"#,
            construct = rs_code.construct,
        ));
    }

    // Emit Rust code.
    fs::write(
        &c.rust_file,
        rustfmt(&format!(
            r#"
// Generated {:?}

use derive_new::new;
use serde::{{Serialize, Deserialize}};
use rand::Rng;
use std::collections::{{HashMap, HashSet}};
use crate::gen::{{Random, GenExt}};

{}

macro_rules! gen {{
  ($encoder:path) =>
{{{{
let mut rng = rand::thread_rng();
let mut ret = Vec::<Vec<u8>>::new();
{}
ret
}}}}
}}
"#,
            chrono::Local::now(),
            rs_definitions.join(""),
            rs_constructs.join("")
        )),
    )
    .unwrap();

    // Emit Python code.
    fs::write(
        &c.python_file,
        yapf(&format!(
            r#"
from dataclasses import dataclass, field
import perde
import typing

{definitions}

TYPES = [{types}]
"#,
            definitions = py_definitions.join("\n"),
            types = py_types.join(","),
        )),
    )
    .unwrap();
}

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt::GenSchema(c) => gen_schema_cmd(c),
        Opt::GenCode(c) => gen_code_cmd(c),
    }
}
