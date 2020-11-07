use crate::{schema::*, types::Object};
use serde::de::{self, DeserializeSeed, Deserializer, Visitor};
use std::fmt;

struct EnumVisitor<'a>(&'a Enum);

impl<'a> EnumVisitor<'a> {
    fn vars(&self) -> Vec<&str> {
        self.0.variants.iter().map(|(v, _)| v.as_ref()).collect()
    }

    fn get<E>(&self, s: &str) -> Result<Object, E>
    where
        E: de::Error,
    {
        if !self.0.variants.contains_key(s) {
            return Err(de::Error::custom(format!(
                "the enum value must be any of {:?}: got `{}`",
                self.vars(),
                s
            )));
        }

        self.0.object.get(s).ok_or(de::Error::custom(format!(
            "cannot construct enum from value {}",
            s
        )))
    }
}

impl<'a, 'de> Visitor<'de> for EnumVisitor<'a> {
    type Value = Object;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an enum value: {:?}", self.vars())
    }

    fn visit_char<E>(self, value: char) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.get(&value.to_string())
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.get(value)
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.get(value)
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.get(&value)
    }
}

impl<'a, 'de> DeserializeSeed<'de> for &'a Enum {
    type Value = Object;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(EnumVisitor(self))
    }
}
