use crate::{error::Convert, schema::*, types::Object};
use serde::de::{DeserializeSeed, Deserializer, SeqAccess, Visitor};
use smallvec::SmallVec;
use std::fmt;

pub struct ListVisitor<'a>(pub &'a List);

impl<'a, 'de> Visitor<'de> for ListVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a list")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut items = SmallVec::<[_; 16]>::new();

        while let Some(value) = seq.next_element_seed(&*self.0.value)? {
            items.push(value);
        }

        let mut list = Object::build_list(items.len()).de()?;

        for (i, a) in items.into_iter().enumerate() {
            list.set(i, a);
        }

        Ok(list.build())
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a List {
    type Value = Object;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(ListVisitor(self))
    }
}
