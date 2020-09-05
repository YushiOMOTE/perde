use serde::de::{DeserializeSeed, Deserializer};
use std::marker::PhantomData;

pub trait DeserializeState<'de, Seed: ?Sized>: Sized {
    fn deserialize_state<D>(seed: &Seed, deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

#[derive(Debug)]
pub struct Seed<S, T> {
    pub seed: S,
    _marker: PhantomData<T>,
}

impl<S, T> Seed<S, T> {
    #[cfg_attr(feature = "perf", flame)]
    pub fn new(seed: S) -> Seed<S, T> {
        Seed {
            seed,
            _marker: PhantomData,
        }
    }
}

impl<'de, 's, S, T> DeserializeSeed<'de> for Seed<&'s S, T>
where
    S: ?Sized,
    T: DeserializeState<'de, S>,
{
    type Value = T;

    #[cfg_attr(feature = "perf", flame)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize_state(self.seed, deserializer)
    }
}
