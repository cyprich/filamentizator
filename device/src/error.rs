use core::fmt::{Display, Write};
use heapless::String;

use super::MAX_STRING_LENGTH;

#[derive(Debug)]
pub enum Error {
    Unknown,
    General(String<MAX_STRING_LENGTH>),
    Reqwless(reqwless::Error),
    SerdeJson(serde_json_core::de::Error),
    Format(core::fmt::Error),
    SsdDisplayError,
}

impl Error {
    pub fn get_type(&self) -> &str {
        match self {
            Error::Unknown => "Unknown Error",
            Error::General(_) => "General Error",
            Error::Reqwless(_) => "Network Error",
            Error::SerdeJson(_) => "Serde Error",
            Error::Format(_) => "Format Error",
            Error::SsdDisplayError => "SSD1306 Display Error",
        }
    }
    pub fn get_description(&self) -> heapless::String<MAX_STRING_LENGTH> {
        let mut s = heapless::String::<MAX_STRING_LENGTH>::new();

        match self {
            Error::Unknown => write!(&mut s, "").unwrap(),
            Error::General(val) => write!(&mut s, "{}", val).unwrap(),
            Error::Reqwless(val) => write!(&mut s, "{}", val).unwrap(),
            Error::SerdeJson(val) => write!(&mut s, "{}", val).unwrap(),
            Error::Format(val) => write!(&mut s, "{}", val).unwrap(),
            Error::SsdDisplayError => todo!(),
        }

        s
    }
}

impl From<String<MAX_STRING_LENGTH>> for Error {
    fn from(value: String<MAX_STRING_LENGTH>) -> Self {
        Self::General(value)
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

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}: {}", self.get_type(), self.get_description())
    }
}
