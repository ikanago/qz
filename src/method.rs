use crate::status::StatusCode;
use std::{convert::TryFrom, fmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
}

impl Default for Method {
    fn default() -> Self {
        Self::Get
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Method::Get => write!(f, "GET"),
            Method::Post => write!(f, "POST"),
        }
    }
}

impl TryFrom<&[u8]> for Method {
    type Error = StatusCode;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match std::str::from_utf8(value) {
            Ok("GET") => Ok(Method::Get),
            Ok("POST") => Ok(Method::Post),
            _ => Err(StatusCode::MethodNotAllowed),
        }
    }
}
