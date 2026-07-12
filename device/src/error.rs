use core::fmt::Display;

use heapless::String;

use super::MAX_STRING_LENGTH;

#[derive(Debug)]
pub enum Error {
    General,
    String(String<MAX_STRING_LENGTH>),
    Reqwless(reqwless::Error),
    SerdeJson(serde_json_core::de::Error),
    Format(core::fmt::Error),
}

impl From<String<MAX_STRING_LENGTH>> for Error {
    fn from(value: String<MAX_STRING_LENGTH>) -> Self {
        Self::String(value)
    }
}

impl From<reqwless::Error> for Error {
    fn from(value: reqwless::Error) -> Self {
        Self::Reqwless(value)
    }
}

impl From<serde_json_core::de::Error> for Error {
    fn from(value: serde_json_core::de::Error) -> Self {
        Self::SerdeJson(value)
    }
}

impl From<core::fmt::Error> for Error {
    fn from(value: core::fmt::Error) -> Self {
        Self::Format(value)
    }
}
