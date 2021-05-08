//! qz is a HTTP server library for my learn. All of the specification of HTTP/1.1 is not covered
//! and not performant enough. The name of qz is comes from "給仕", which means waiter in Japanese.
//!
//! # Getting Started
//!
//! To build app with this library, first add following to `Cargo.toml`:
//! ```toml
//! qz = "0.1"
//! tokio = { version = "1.5", features = ["macros"] }
//! serde = { version = "1.0", features = ["derive"] }
//! ```
//!
//! # Examples
//!
//! ```no_run
//! use qz::{method::Method, request::Request, response::Response, server::Server};
//! use std::io;
//!
//! async fn hello(_request: Request, _: ()) -> impl Into<Response> {
//!     "hello"
//! }
//!
//! #[tokio::main]
//! async fn main() -> io::Result<()> {
//!     let server = Server::builder()
//!         .route("/hello", Method::Get, hello)
//!         .build();
//!     Server::run(server, 8080).await
//! }
//! ```
//!
//! ```sh
//! $ curl localhost:8080/hello
//! Hello
//! ```
//!

pub mod body;
pub mod handler;
pub mod header;
pub mod method;
pub mod middleware;
pub mod mime;
mod parser;
pub mod redirect;
pub mod request;
pub mod response;
mod router;
pub mod server;
pub mod static_files;
pub mod status;

use crate::status::StatusCode;
use std::{
    convert::{From, TryFrom},
    fmt, str,
};

/// All errornous function returns this type. because all error in this crate converges HTTP error which is
/// represented by status code.
pub type Result<T, E = StatusCode> = std::result::Result<T, E>;

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
