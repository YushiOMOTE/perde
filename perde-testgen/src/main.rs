use perde_testgen::{gen, gen_schema, Code, Rust};

pub fn gen_rust_code(num: usize, depth: usize) -> Vec<Code> {
    (0..num)
        .map(|_| {
            let s = gen_schema(depth);
            gen(Rust, &s)
        })
        .collect()
}

fn main() {
    let mut constructs = vec![];
    let mut definitions = vec![];

    for code in gen_rust_code(100, 5) {
        definitions.push(code.definitions);

        constructs.push(format!(
            r#"
  {construct}
  let f = File::create("json/{typename}.json").unwrap();
  serde_json::to_writer(f, &v).unwrap();
"#,
            construct = code.construct,
            typename = code
                .typename
                .replace("(", "_")
                .replace(")", "_")
                .replace("<", "_")
                .replace(">", "_")
                .replace(", ", "_")
        ));
    }

    println!(
        r#"
mod gen;

use derive_new::new;
use serde::{{Serialize, Deserialize}};
use rand::Rng;
use std::fs::File;
use std::collections::{{HashMap, HashSet}};
use crate::gen::{{Random, GenExt}};

{}

fn main() {{
let mut rng = rand::thread_rng();
{}
}}
"#,
        definitions.join(""),
        constructs.join("")
    );
}
