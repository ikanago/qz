use crate::status::StatusCode;
use std::{convert::TryFrom, fmt};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Method {
    Get,
    Post,
    Options,
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
            Method::Options => write!(f, "OPTIONS"),
        }
    }
}

impl TryFrom<&[u8]> for Method {
    type Error = StatusCode;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match std::str::from_utf8(value) {
            Ok("GET") => Ok(Method::Get),
            Ok("POST") => Ok(Method::Post),
            Ok("OPTONS") => Ok(Method::Options),
            _ => Err(StatusCode::MethodNotAllowed),
        }
    }
}
