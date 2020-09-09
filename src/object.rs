use crate::{schema::Schema, util::*};
use pyo3::prelude::*;

pub struct TypedObject<'a> {
    pub schema: &'a Schema,
    pub object: &'a PyAny,
}

impl<'a> TypedObject<'a> {
    pub fn new(schema: &'a Schema, object: &'a PyAny) -> Self {
        Self { schema, object }
    }
}
