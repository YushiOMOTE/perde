use codegen::{gen, gen_schema, Code, Python, Rust};
use std::{
    borrow::Cow,
    fs,
    io::prelude::*,
    path::PathBuf,
    process::{Command, Stdio},
};
use structopt::StructOpt;

pub fn gen_code(num: usize, depth: usize) -> Vec<(Code, Code)> {
    (0..num)
        .map(|_| {
            let s = gen_schema(depth);
            (gen(Rust, &s), gen(Python, &s))
        })
        .collect()
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
struct Opt {
    /// Path to rust file.
    rust_file: PathBuf,
    /// Path to python file.
    python_file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let mut rs_constructs = vec![];
    let mut rs_definitions = vec![];
    let mut py_definitions = vec![];
    let mut py_types = vec![];

    for (rs_code, py_code) in gen_code(100, 5) {
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
        &opt.rust_file,
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
        &opt.python_file,
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
