use std::convert::TryFrom;
use std::str;
use crate::parser::ParseError;

pub type Uri = Vec<u8>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Version {
    OneDotOne,
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
