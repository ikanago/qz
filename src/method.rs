use std::convert::TryFrom;

use crate::parser::ParseError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
}

impl TryFrom<&[u8]> for Method {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match std::str::from_utf8(value) {
            Ok("GET") => Ok(Method::Get),
            _ => Err(Self::Error::InvalidMethod),
        }
    }
}

