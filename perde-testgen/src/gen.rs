use crate::schema::*;
use indexmap::IndexMap;

pub trait CodeGen {
    fn define_enum(&mut self, u: Union, context: &mut Context) -> String;

    fn define_class(&mut self, c: Class, context: &mut Context) -> String;

    fn construct(&mut self, schema: &Schema) -> String;

    fn gen(&mut self, schema: &Schema, context: &mut Context) -> (String, String);
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

    let (typename, construct) = codegen.gen(s, &mut context);

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
