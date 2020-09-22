use serde::{de, ser};
use std::fmt::{self, Display};

pub type Result<T> = std::result::Result<T, Error>;

pub trait Convert<T> {
    fn de<E>(self) -> std::result::Result<T, E>
    where
        E: de::Error,
        Self: Sized;

    fn ser<E>(self) -> std::result::Result<T, E>
    where
        E: ser::Error,
        Self: Sized;
}

#[derive(Debug)]
pub struct Error(anyhow::Error);

impl std::ops::Deref for Error {
    type Target = anyhow::Error;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Self(e)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

impl<T> Convert<T> for std::result::Result<T, Error> {
    fn de<E>(self) -> std::result::Result<T, E>
    where
        E: de::Error,
        Self: Sized,
    {
        self.map_err(|e| de::Error::custom(e.to_string()))
    }

    fn ser<E>(self) -> std::result::Result<T, E>
    where
        E: ser::Error,
        Self: Sized,
    {
        self.map_err(|e| ser::Error::custom(e.to_string()))
    }
}

macro_rules! err {
    ($($t:tt)*) => {
        crate::error::Error::from(anyhow::anyhow!($($t)*))
    }
}

macro_rules! erret {
    ($($t:tt)*) => {
        return Err(crate::error::Error::from(anyhow::anyhow!($($t)*)))
    }
}
