use codegen::{gen, gen_schema, Code, Rust};
use std::{
    borrow::Cow,
    io::prelude::*,
    process::{Command, Stdio},
};

pub fn gen_rust_code(num: usize, depth: usize) -> Vec<Code> {
    (0..num)
        .map(|_| {
            let s = gen_schema(depth);
            gen(Rust, &s)
        })
        .collect()
}

fn rustfmt(value: &str) -> String {
    let mut process = Command::new("rustfmt")
        .arg("--emit=stdout")
        .arg("--edition=2018")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();

    let stdin = process.stdin.as_mut().unwrap();
    stdin.write_all(value.as_bytes()).unwrap();

    let output = process.wait_with_output().unwrap();
    assert!(output.status.success());

    std::str::from_utf8(&output.stdout).unwrap().to_owned()
}

fn main() {
    let mut constructs = vec![];
    let mut definitions = vec![];

    for code in gen_rust_code(100, 5) {
        definitions.push(code.definitions);

        constructs.push(format!(
            r#"
  {construct}
  println!("======================\n");
  println!("{{}}", serde_json::to_string(&v).unwrap());
"#,
            construct = code.construct,
        ));
    }

    println!(
        r#"
// Generated {:?}
mod gen;

use derive_new::new;
use serde::{{Serialize, Deserialize}};
use rand::Rng;
use std::collections::{{HashMap, HashSet}};
use crate::gen::{{Random, GenExt}};

{}

fn main() {{
let mut rng = rand::thread_rng();
{}
}}
"#,
        chrono::Local::now(),
        definitions.join(""),
        constructs.join("")
    );
}
