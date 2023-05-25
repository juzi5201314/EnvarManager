use serde::Serializer;
use std::fmt::Display;

#[derive(Debug)]
pub struct Error(String);

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<E> From<E> for Error
where
    E: Display,
{
    fn from(value: E) -> Self {
        Error(value.to_string())
    }
}
