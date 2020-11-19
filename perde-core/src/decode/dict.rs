use crate::{error::Convert, object::Object, schema::*};
use serde::de::{DeserializeSeed, Deserializer, MapAccess, Visitor};
use std::fmt;

pub struct DictVisitor<'a>(pub &'a Dict);

impl<'a, 'de> Visitor<'de> for DictVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a map")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut dict = Object::build_dict().de()?;

        while let Some(key) = access.next_key_seed(&*self.0.key)? {
            let key: Object = key;
            let value: Object = access.next_value_seed(&*self.0.value)?;
            dict.set(key, value).de()?;
        }

        Ok(dict.build())
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a Dict {
    type Value = Object;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(DictVisitor(self))
    }
}
