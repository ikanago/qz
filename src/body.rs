use std::convert::From;

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

impl From<&str> for Body {
    fn from(value: &str) -> Self {
        Self::Some(Vec::from(value.as_bytes()))
    }
}
