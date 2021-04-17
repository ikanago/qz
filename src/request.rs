use crate::{
    method::Method,
    parser::{ParseError, Parser},
};
use std::convert::TryFrom;
use std::str;

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

#[derive(Debug)]
pub struct Request {
    method: Method,
    uri: Uri,
    version: Version,
}

impl Request {
    pub fn new(request_bytes: &[u8]) -> Result<Self, ParseError> {
        let mut p = Parser::new(request_bytes);
        let (method, uri, version) = p.parse_request_line()?;
        Ok(Self {
            method,
            uri,
            version,
        })
    }
}
