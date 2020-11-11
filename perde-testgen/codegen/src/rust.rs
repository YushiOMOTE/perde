use crate::{
    gen::{Code, CodeGen, Context, Hint},
    schema::*,
};

pub struct Rust;

impl CodeGen for Rust {
    fn define_enum(&mut self, u: Union, hint: &Hint, context: &mut Context) -> String {
        let mut s = "".to_string();

        let mut derives = vec!["Serialize", "Deserialize", "Debug", "Clone", "new"];
        if hint.eq {
            derives.push("PartialEq");
            derives.push("Eq");
        }
        if hint.hash {
            derives.push("Hash");
        }
        s.push_str(&format!("#[derive({})]\n", derives.join(",")));
        s.push_str("#[serde(untagged)]\n");
        s.push_str(&format!("pub enum {} {{\n", u.name));
        for (i, v) in u.variants.iter().enumerate() {
            s.push_str(&format!(
                "  {}({}),\n",
                (i as u8 + 'A' as u8) as char,
                self.gen(&v, context)
            ));
        }
        s.push_str("}\n");
        s.push_str("\n");
        s.push_str(&format!("impl Random for {} {{\n", u.name));
        s.push_str(&format!(
            "  fn random<R: Rng + ?Sized>(rng: &mut R) -> {} {{\n",
            u.name
        ));
        s.push_str(&format!(
            "    let v: usize = rng.gen_range(0, {});\n",
            u.variants.len()
        ));
        s.push_str("    match v {\n");
        for (i, v) in u.variants.iter().enumerate() {
            s.push_str(&format!(
                "      {} => Self::{}({{ let v: {} = {}; v }}),\n",
                i,
                (i as u8 + 'A' as u8) as char,
                self.gen(&v, context),
                self.construct(v)
            ));
        }
        s.push_str("      _ => unreachable!(),\n");
        s.push_str("    }\n");
        s.push_str("  }\n");
        s.push_str("}\n");

        context.register(u.name, s.clone());

        s
    }

    fn define_class(&mut self, c: Class, hint: &Hint, context: &mut Context) -> String {
        let mut derives = vec!["Serialize", "Deserialize", "Debug", "Clone", "new"];
        if hint.eq {
            derives.push("PartialEq");
            derives.push("Eq");
        }
        if hint.hash {
            derives.push("Hash");
        }
        if hint.default {
            derives.push("Default");
        }
        let mut s = format!("#[derive({})]\n", derives.join(","));

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
        s.push_str("#[allow(unused)]");
        s.push_str(&format!("impl Random for {} {{\n", c.name));
        s.push_str(&format!(
            "  fn random<R: Rng + ?Sized>(rng: &mut R) -> {} {{\n",
            c.name
        ));
        s.push_str(&format!("    {}::new(\n", c.name));
        for (name, f) in &c.fields {
            s.push_str(&format!("      {},\n", self.construct(&f.schema)));
        }
        s.push_str("    )\n");
        s.push_str("  }\n");
        s.push_str("}\n");

        context.register(c.name.clone(), s);

        c.name.clone()
    }

    fn construct(&mut self, schema: &Schema) -> String {
        "rng.gen_ext()".into()
    }

    fn construct_line(&mut self, typename: &str, schema: &Schema) -> String {
        format!("let v: {} = {};", typename, self.construct(schema))
    }

    fn gen(&mut self, schema: &Schema, context: &mut Context) -> String {
        let hint = Hint::new(schema.has_eq(), schema.has_hash(), schema.has_default());

        let typename = match schema {
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
            Schema::Class(c) => {
                self.define_class(c.clone(), &hint, context);
                c.name.clone()
            }
            Schema::Enum(e) => unimplemented!(),
            Schema::Optional(o) => format!("Option<{}>", self.gen(&o.value, context)),
            Schema::Union(u) => {
                self.define_enum(u.clone(), &hint, context);
                u.name.clone()
            }
        };

        typename
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::gen::gen;
    use rand::Rng;

    fn gen_rust_code(s: &Schema) -> Code {
        gen(Rust, s)
    }

    #[test]
    fn random_gen() {
        for _ in 0..50 {
            let schema: Schema = gen_schema(5);
            let code = gen_rust_code(&schema);
            println!("--- {} ---", code.typename);
            println!("{}", code.definitions);
            println!("{}", code.construct);
            println!("----------");
        }
    }
}
