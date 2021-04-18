pub mod header;
pub mod method;
mod parser;
pub mod request;
pub mod server;

use crate::parser::ParseError;
use std::{convert::TryFrom, fmt, str};

#[derive(Debug, PartialEq, Eq)]
pub struct Uri(Vec<u8>);

impl Uri {
    pub fn new(path: &[u8]) -> Self {
        Self(path.to_vec())
    }
}

impl Default for Uri {
    fn default() -> Self {
        Uri(b"/".to_vec())
    }
}

impl fmt::Display for Uri {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", str::from_utf8(&self.0).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Version {
    OneDotOne,
}

impl Default for Version {
    fn default() -> Self {
        Self::OneDotOne
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Version::OneDotOne => write!(f, "1.1"),
        }
    }
}

impl TryFrom<&[u8]> for Version {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match str::from_utf8(value) {
            Ok("1.1") => Ok(Version::OneDotOne),
            _ => Err(ParseError::InvalidVersion),
        }
    }
}
