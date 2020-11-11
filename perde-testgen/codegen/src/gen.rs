use crate::schema::*;
use derive_new::new;
use indexmap::IndexMap;

#[derive(new, Clone, Debug)]
pub struct Hint {
    pub eq: bool,
    pub hash: bool,
    pub default: bool,
}

pub trait CodeGen {
    fn define_enum(&mut self, u: Union, hint: &Hint, context: &mut Context) -> String;

    fn define_class(&mut self, c: Class, hint: &Hint, context: &mut Context) -> String;

    fn construct(&mut self, schema: &Schema) -> String;

    fn construct_line(&mut self, typename: &str, schema: &Schema) -> String;

    fn gen(&mut self, schema: &Schema, context: &mut Context) -> String;
}

pub struct Context {
    types: IndexMap<String, String>,
}

impl Context {
    fn new() -> Self {
        Self {
            types: IndexMap::new(),
        }
    }

    pub fn register(&mut self, typename: String, definition: String) {
        self.types.insert(typename, definition);
    }
}

#[derive(Debug, Clone)]
pub struct Code {
    pub typename: String,
    pub construct: String,
    pub definitions: String,
}

pub fn gen<T: CodeGen>(mut codegen: T, s: &Schema) -> Code {
    let mut context = Context::new();

    let mut definitions = "".to_string();

    let typename = codegen.gen(s, &mut context);
    let construct = codegen.construct_line(&typename, s);

    for (_, deps) in &context.types {
        definitions.push_str(deps);
        definitions.push_str("\n");
    }

    Code {
        typename,
        construct,
        definitions,
    }
}
