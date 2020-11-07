use perde_testgen::{gen, gen_schema, Code, Rust};

pub fn gen_rust_code(num: usize, depth: usize) -> Vec<Code> {
    (0..num)
        .map(|_| {
            let s = gen_schema(depth);
            gen(Rust, &s)
        })
        .collect()
}

fn main() {}
