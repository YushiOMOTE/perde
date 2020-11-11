use crate::{
    gen::{Code, CodeGen, Context, Hint},
    schema::*,
};

pub struct Python;

impl CodeGen for Python {
    fn define_enum(&mut self, u: Union, hint: &Hint, context: &mut Context) -> String {
        unreachable!()
    }

    fn define_class(&mut self, c: Class, hint: &Hint, context: &mut Context) -> String {
        let mut s = "".to_string();

        let mut cls_attr = vec![];
        if let Some(rename_all) = &c.attr.rename_all {
            cls_attr.push(format!("rename_all = \"{}\"", rename_all));
        }
        if let Some(rename) = &c.attr.rename {
            cls_attr.push(format!("rename = \"{}\"", rename));
        }
        if c.attr.deny_unknown_fields {
            cls_attr.push("deny_unknown_fields = True".into());
        }
        if c.attr.default {
            cls_attr.push("default = True".into());
        }
        if !cls_attr.is_empty() {
            s.push_str(&format!("@perde.attr({})\n", cls_attr.join(", ")))
        }
        s.push_str("@dataclass\n");
        s.push_str(&format!("class {}:\n", c.name));

        for (name, f) in &c.fields {
            let mut field_attr = vec![];
            if f.attr.flatten {
                field_attr.push("\"perde_flatten\": True".to_string());
            }
            if let Some(rename) = &f.attr.rename {
                field_attr.push(format!("\"perde_rename\": \"{}\"", rename));
            }
            if f.attr.skip {
                field_attr.push("\"perde_skip\": True".into());
            }
            if f.attr.skip_deserializing {
                field_attr.push("\"perde_skip_deserializing\": True".into());
            }

            let field = if !field_attr.is_empty() {
                format!(" = field(metadata = {{{}}})", field_attr.join(", "))
            } else {
                "".into()
            };

            s.push_str(&format!(
                "  {}: {}{}\n",
                name,
                self.gen(&f.schema, context),
                field
            ));
        }
        if c.fields.is_empty() {
            s.push_str("  pass\n");
        }
        s.push_str("\n");
        s.push_str("\n");

        context.register(c.name.clone(), s);

        c.name.clone()
    }

    fn construct(&mut self, schema: &Schema) -> String {
        "".into()
    }

    fn construct_line(&mut self, typename: &str, schema: &Schema) -> String {
        "".into()
    }

    fn gen(&mut self, schema: &Schema, context: &mut Context) -> String {
        let hint = Hint::new(schema.has_eq(), schema.has_hash(), schema.has_default());

        let typename = match schema {
            Schema::Bool => "bool".into(),
            Schema::Int => "int".into(),
            Schema::Float => "float".into(),
            Schema::Str => "str".into(),
            Schema::Bytes => "bytes".into(),
            Schema::Dict(d) => format!(
                "typing.Dict[{}, {}]",
                self.gen(&d.key, context),
                self.gen(&d.value, context),
            ),
            Schema::List(l) => format!("typing.List[{}]", self.gen(&l.value, context)),
            Schema::Set(s) => format!("typing.Set[{}]", self.gen(&s.value, context)),
            Schema::Tuple(t) => {
                let s: Vec<_> = t
                    .args
                    .iter()
                    .map(|t| format!("{}", self.gen(&t, context)))
                    .collect();

                format!(
                    "typing.Tuple[{}]",
                    if s.is_empty() {
                        "()".into()
                    } else {
                        s.join(", ")
                    }
                )
            }
            Schema::Class(c) => {
                self.define_class(c.clone(), &hint, context);
                c.name.clone()
            }
            Schema::Enum(e) => unimplemented!(),
            Schema::Optional(o) => format!("typing.Optional[{}]", self.gen(&o.value, context)),
            Schema::Union(u) => {
                let s: Vec<_> = u
                    .variants
                    .iter()
                    .map(|t| format!("{}", self.gen(&t, context)))
                    .collect();
                format!("typing.Union[{}]", s.join(", "))
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

    fn gen_python_code(s: &Schema) -> Code {
        gen(Python, s)
    }

    #[test]
    fn random_python_gen() {
        for _ in 0..50 {
            let schema: Schema = gen_schema(5);
            let code = gen_python_code(&schema);
            println!("--- {} ---", code.typename);
            println!("{}", code.definitions);
            println!("{}", code.construct);
            println!("----------");
        }
    }
}
