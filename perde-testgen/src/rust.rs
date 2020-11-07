use crate::schema::*;
use indexmap::IndexMap;

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

    fn define_enum(&mut self, u: Union) -> String {
        let name = format!("Enum{}", self.enumid);
        self.enumid += 1;

        let mut s = "".to_string();
        s.push_str("#[derive(Serialize, Deserialize, Debug, Clone)]\n");
        s.push_str("#[serde(untagged)]\n");
        s.push_str(&format!("pub enum {} {{\n", name));
        for v in &u.variants {
            s.push_str(&format!("  {},\n", v.to_rust_code(self)));
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
                v.to_rust_code(self),
                gen
            ));
        }
        s.push_str("    }\n");
        s.push_str("  }\n");
        s.push_str("}\n");

        self.types.insert(name.clone(), s);

        name
    }

    fn define_class(&mut self, c: Class) -> String {
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
            s.push_str(&format!("  {}: {},\n", name, f.schema.to_rust_code(self)));
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

        self.types.insert(c.name.clone(), s);

        c.name.clone()
    }
}

trait ToRustCode {
    fn to_rust_code(&self, context: &mut Context) -> String;
}

impl ToRustCode for Schema {
    fn to_rust_code(&self, context: &mut Context) -> String {
        match self {
            Self::Bool => "bool".into(),
            Self::Int => "i64".into(),
            Self::Float => "f64".into(),
            Self::Str => "String".into(),
            Self::Bytes => "Vec<u8>".into(),
            Self::Dict(d) => format!(
                "HashMap<{}, {}>",
                d.key.to_rust_code(context),
                d.value.to_rust_code(context)
            ),
            Self::List(l) => format!("Vec<{}>", l.value.to_rust_code(context)),
            Self::Set(s) => format!("HashSet<{}>", s.value.to_rust_code(context)),
            Self::Tuple(t) => {
                let mut s = "(".to_string();
                for t in &t.args {
                    s.push_str(&format!("{}, ", t.to_rust_code(context)));
                }
                s.push_str(")");
                s
            }
            Self::Class(c) => context.define_class(c.clone()),
            Self::Enum(e) => unimplemented!(),
            Self::Optional(o) => format!("Option<{}>", o.value.to_rust_code(context)),
            Self::Union(u) => context.define_enum(u.clone()),
        }
    }
}

pub fn to_rust_code(s: &Schema) -> (String, String) {
    let mut context = Context::new();
    let mut code = "".to_string();

    let s = s.to_rust_code(&mut context);

    for (_, deps) in &context.types {
        code.push_str(deps);
        code.push_str("\n");
    }

    (s, code)
}

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

        let (_, code) = to_rust_code(&s);
        println!("{}", code);
    }

    #[test]
    fn more_gen() {
        let mut rng = rand::thread_rng();

        for _ in 0..50 {
            let schema: Schema = gen_schema(5);
            let (s, code) = to_rust_code(&schema);
            println!("--- {} ---", s);
            println!("{}", code);
            println!("----------");
        }
    }
}
