use std::{
    io::prelude::*,
    path::PathBuf,
    process::{Command, Stdio},
};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    /// Base directory.
    #[structopt(name = "base")]
    base: PathBuf,
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

fn main() {
    let opt = Opt::from_args();

    let mut rust_code = Vec::new();

    for e in walkdir::WalkDir::new(opt.base)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().filter(|&s| s == "py").is_some())
    {
        let content = match std::fs::read(e.path()) {
            Ok(c) => c,
            Err(c) => {
                eprintln!("codegen: skipping {}: {}", e.path().display(), c);
                continue;
            }
        };
        eprintln!("codegen: scanning {}", e.path().display());

        let mut is_comment = false;
        let mut comment = String::new();
        for l in content.lines().filter_map(|l| l.ok()) {
            if l.starts_with(r#""""rust"#) {
                is_comment = true;
                continue;
            } else if l.starts_with(r#"""""#) {
                if is_comment {
                    rust_code.push(comment.split_off(0));
                }
                is_comment = false;
            }
            if is_comment {
                comment.push_str(&l);
                comment.push_str("\n");
            }
        }
    }

    let code = format!(
        r#"
// This file is generated. Don't modify manually.

{util}

use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {{
  /// Output directory
  #[structopt(name = "dir")]
  dir: PathBuf,
}}

fn main() {{
  {code}

  let opt = Opt::from_args();
  save(&opt.dir);
}}
"#,
        util = include_str!("util.rs"),
        code = rust_code.join("\n\n")
    );

    if std::panic::catch_unwind(|| {
        println!("{}", rustfmt(&code));
    })
    .is_err()
    {
        eprintln!("****** Broken code:\n\n {}", code);
        eprintln!("******");
    }
}
