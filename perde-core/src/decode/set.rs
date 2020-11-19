use crate::{error::Convert, object::Object, schema::*};
use serde::de::{DeserializeSeed, Deserializer, SeqAccess, Visitor};
use std::fmt;

pub struct SetVisitor<'a>(pub &'a Set);

impl<'a, 'de> Visitor<'de> for SetVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a set")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut set = Object::build_set().de()?;

        while let Some(value) = seq.next_element_seed(&*self.0.value)? {
            set.set(value).de()?;
        }

        Ok(set.build())
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a Set {
    type Value = Object;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(SetVisitor(self))
    }
}
