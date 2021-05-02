use std::{fmt, convert::From};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Body {
    None,
    Some(Vec<u8>),
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

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Body::Some(bytes) => match std::str::from_utf8(&bytes) {
                Ok(s) => write!(f, "{}", s),
                Err(_) => write!(f, "{:?}", bytes),
            }
            Body::None => write!(f, ""),
        }
    }
}
