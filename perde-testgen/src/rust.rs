use crate::schema::*;
use indexmap::IndexMap;

trait CodeGen {
    fn define_enum(&mut self, u: Union, context: &mut Context) -> (String, String);

    fn define_class(&mut self, c: Class, context: &mut Context) -> String;

    fn gen(&mut self, schema: &Schema, context: &mut Context) -> String;
}

struct Context {
    enumid: usize,
    types: IndexMap<String, String>,
}

impl Context {
    fn new() -> Self {
        Self {
            enumid: 0,
            types: IndexMap::new(),
        }
    }
}

struct Rust;

impl CodeGen for Rust {
    fn define_enum(&mut self, u: Union, context: &mut Context) -> (String, String) {
        let name = format!("Enum{}", context.enumid);
        context.enumid += 1;

        let mut s = "".to_string();
        s.push_str("#[derive(Serialize, Deserialize, Debug, Clone)]\n");
        s.push_str("#[serde(untagged)]\n");
        s.push_str(&format!("pub enum {} {{\n", name));
        for v in &u.variants {
            s.push_str(&format!("  {},\n", self.gen(&v, context)));
        }
        s.push_str("}\n");
        s.push_str("\n");
        s.push_str(&format!("impl Distribution<{}> for Standard {{\n", name));
        s.push_str(&format!(
            "  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> {} {{\n",
            name
        ));
        s.push_str(&format!(
            "    let v: usize = rng.gen_range(0, {});\n",
            u.variants.len()
        ));
        s.push_str("    match v {\n");
        for (i, v) in u.variants.iter().enumerate() {
            let gen = match v {
                Schema::Bytes => "gen_vec(rng)",
                Schema::Dict(_) => "gen_map(rng)",
                Schema::List(_) => "gen_vec(rng)",
                Schema::Set(_) => "gen_set(rng)",
                Schema::Optional(_) => "gen_opt(rng)",
                _ => "rng.gen()",
            };
            s.push_str(&format!(
                "      {} => {{ let v: {} = {}; v }},\n",
                i,
                self.gen(&v, context),
                gen
            ));
        }
        s.push_str("      _ => unreachable!(),\n");
        s.push_str("    }\n");
        s.push_str("  }\n");
        s.push_str("}\n");

        context.types.insert(name.clone(), s.clone());

        (name, s)
    }

    fn define_class(&mut self, c: Class, context: &mut Context) -> String {
        let mut s = "#[derive(Serialize, Deserialize, Debug, Clone, new)]\n".to_string();

        let mut cls_attr = vec![];
        if let Some(rename_all) = &c.attr.rename_all {
            cls_attr.push(format!("rename_all = \"{}\"", rename_all));
        }
        if let Some(rename) = &c.attr.rename {
            cls_attr.push(format!("rename = \"{}\"", rename));
        }
        if c.attr.deny_unknown_fields {
            cls_attr.push("deny_unknown_fields".into());
        }
        if c.attr.default {
            cls_attr.push("default".into());
        }
        if !cls_attr.is_empty() {
            s.push_str(&format!("#[serde({})]\n", cls_attr.join(", ")));
        }

        s.push_str(&format!("pub struct {} {{\n", c.name));
        for (name, f) in &c.fields {
            let mut field_attr = vec![];
            if f.attr.flatten {
                field_attr.push("flatten".to_string());
            }
            if let Some(rename) = &f.attr.rename {
                field_attr.push(format!("rename = \"{}\"", rename));
            }
            if f.attr.skip {
                field_attr.push(format!("skip"));
            }
            if f.attr.skip_deserializing {
                field_attr.push(format!("skip_deserializing"));
            }
            if !field_attr.is_empty() {
                s.push_str(&format!("  #[serde({})]\n", field_attr.join(", ")))
            }
            s.push_str(&format!("  {}: {},\n", name, self.gen(&f.schema, context)));
        }
        s.push_str("}\n");
        s.push_str("\n");
        s.push_str(&format!("impl Distribution<{}> for Standard {{\n", c.name));
        s.push_str(&format!(
            "  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> {} {{\n",
            c.name
        ));
        s.push_str(&format!("    {}::new(\n", c.name));
        for (name, f) in &c.fields {
            let gen = match f.schema {
                Schema::Bytes => "gen_vec(rng)",
                Schema::Dict(_) => "gen_map(rng)",
                Schema::List(_) => "gen_vec(rng)",
                Schema::Set(_) => "gen_set(rng)",
                Schema::Optional(_) => "gen_opt(rng)",
                _ => "rng.gen()",
            };
            s.push_str(&format!("      {},\n", gen));
        }
        s.push_str("    )\n");
        s.push_str("  }\n");
        s.push_str("}\n");

        context.types.insert(c.name.clone(), s);

        c.name.clone()
    }

    fn gen(&mut self, schema: &Schema, context: &mut Context) -> String {
        match schema {
            Schema::Bool => "bool".into(),
            Schema::Int => "i64".into(),
            Schema::Float => "f64".into(),
            Schema::Str => "String".into(),
            Schema::Bytes => "Vec<u8>".into(),
            Schema::Dict(d) => format!(
                "HashMap<{}, {}>",
                self.gen(&d.key, context),
                self.gen(&d.value, context),
            ),
            Schema::List(l) => format!("Vec<{}>", self.gen(&l.value, context)),
            Schema::Set(s) => format!("HashSet<{}>", self.gen(&s.value, context)),
            Schema::Tuple(t) => {
                let mut s = "(".to_string();
                for t in &t.args {
                    s.push_str(&format!("{}, ", self.gen(&t, context)));
                }
                s.push_str(")");
                s
            }
            Schema::Class(c) => self.define_class(c.clone(), context),
            Schema::Enum(e) => unimplemented!(),
            Schema::Optional(o) => format!("Option<{}>", self.gen(&o.value, context)),
            Schema::Union(u) => self.define_enum(u.clone(), context).0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Code {
    pub typename: String,
    pub definitions: String,
}

fn gen<T: CodeGen>(mut codegen: T, s: &Schema) -> Code {
    let mut context = Context::new();

    let mut definitions = "".to_string();

    let typename = codegen.gen(s, &mut context);

    for (_, deps) in &context.types {
        definitions.push_str(deps);
        definitions.push_str("\n");
    }

    Code {
        typename,
        definitions,
    }
}

pub fn gen_rust_code(s: &Schema) -> Code {
    gen(Rust, s)
}

// pub fn gen_python_code(s: &Schema) -> Code {
//     gen(Python, s)
// }

#[cfg(test)]
mod test {
    use super::*;
    use rand::Rng;

    macro_rules! map {
        ($($key:expr => $value:expr),*) => {{
            let mut map = IndexMap::new();
            $(map.insert({$key}.into(), {$value}.into());)*
                map
        }}
    }

    #[test]
    fn struct_gen() {
        let s = Schema::Class(Class::new(
            "A".into(),
            ClassAttr::default(),
            map!(
                "a" => FieldSchema::new(FieldAttr::default(), Schema::Bool),
                "b" => FieldSchema::new(FieldAttr::default(), Schema::Class(Class::new(
                    "C".into(),
                    ClassAttr::default(),
                    map!(
                        "x" => FieldSchema::new(FieldAttr::default(), Schema::Dict(Dict::new(
                            Box::new(Schema::Bool), Box::new(Schema::Int)
                        ))),
                        "y" => FieldSchema::new(FieldAttr::default(), Schema::Bytes)
                    )
                ))),
                "c" => FieldSchema::new(
                    FieldAttr::new(true, None, false ,false ,false),
                    Schema::Union(Union::new(vec![Schema::Bool, Schema::Int, Schema::Str]))
                )
            ),
        ));

        let code = gen_rust_code(&s);
        println!("{}", code.definitions);
    }

    #[test]
    fn more_gen() {
        let mut rng = rand::thread_rng();

        for _ in 0..50 {
            let schema: Schema = gen_schema(5);
            let code = gen_rust_code(&schema);
            println!("--- {} ---", code.typename);
            println!("{}", code.definitions);
            println!("----------");
        }
    }
}
