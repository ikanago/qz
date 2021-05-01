pub mod body;
pub mod handler;
pub mod header;
pub mod method;
mod mime;
mod parser;
pub mod redirect;
pub mod request;
pub mod response;
mod router;
pub mod server;
pub mod static_files;
pub mod status;

use crate::status::StatusCode;
use std::{convert::{From, TryFrom}, fmt, str};

/// All errornous function returns this type. because all error in this crate converges HTTP error which is
/// represented by status code.
pub type Result<T> = std::result::Result<T, StatusCode>;

#[derive(Debug, PartialEq, Eq)]
pub struct Uri(Vec<u8>);

impl Uri {
    pub fn new(path: &[u8]) -> Self {
        Self(path.to_vec())
    }
}

impl From<&str> for Uri {
    fn from(s: &str) -> Self {
        Uri(s.as_bytes().to_vec())
    }
}

impl AsRef<[u8]> for Uri {
    fn as_ref(&self) -> &[u8] {
        &self.0
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Version {
    OneDotOne,
}

impl Version {
    fn as_bytes(&self) -> &'static [u8; 3] {
        match &self {
            Version::OneDotOne => b"1.1",
        }
    }
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
    type Error = StatusCode;
    fn try_from(value: &[u8]) -> Result<Self> {
        match value {
            b"1.1" => Ok(Version::OneDotOne),
            _ => Err(StatusCode::HttpVersionNotSupported),
        }
    }
}
