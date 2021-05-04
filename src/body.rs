use std::{convert::From, fmt};
use crate::{response::Response, status::StatusCode};
use serde::de::DeserializeOwned;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Body {
    None,
    Some(Vec<u8>),
}

impl Body {
    pub fn len(&self) -> usize {
        match &self {
            Body::Some(bytes) => bytes.len(),
            Body::None => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_some(&self) -> bool {
        match &self {
            Body::Some(_) => true,
            Body::None => false,
        }
    }

    pub fn is_none(&self) -> bool {
        match &self {
            Body::Some(_) => false,
            Body::None => true,
        }
    }

    pub fn into_form<T: DeserializeOwned>(&self) -> crate::Result<T> {
        match &self {
            Body::Some(bytes) => serde_urlencoded::from_bytes::<T>(bytes).or(Err(StatusCode::BadRequest)),
            Body::None => Err(StatusCode::BadRequest),
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::None
    }
}

impl AsRef<[u8]> for Body {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::None => &[],
            Self::Some(bytes) => bytes,
        }
    }
}

impl From<String> for Body {
    fn from(s: String) -> Self {
        Self::Some(s.into())
    }
}

impl<'a> From<&'a str> for Body {
    fn from(s: &'a str) -> Self {
        Self::Some(s.into())
    }
}

impl From<Vec<u8>> for Body {
    fn from(bytes: Vec<u8>) -> Self {
        Self::Some(bytes)
    }
}

impl From<&[u8]> for Body {
    fn from(bytes: &[u8]) -> Self {
        Self::Some(bytes.into())
    }
}

impl From<Body> for Response {
    fn from(body: Body) -> Self {
        Response::builder().set_body(body).build()
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Body::Some(bytes) => match std::str::from_utf8(&bytes) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => write!(f, "{:?}", bytes),
            },
            Body::None => write!(f, ""),
        }
    }
}
