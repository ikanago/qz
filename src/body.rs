use crate::{response::Response, status::StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::{convert::From, fmt};

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

    /// Parse `Body` as `application/x-www-urlencoded` data.
    ///
    /// # Examples
    /// ```rust
    /// use qz::body::Body;
    /// use serde::Deserialize;
    /// #[derive(Deserialize)]
    /// struct User {
    ///     username: String,
    ///     password: String,
    /// }
    ///
    /// let body = Body::from("username=John&password=qwerty");
    /// let user: User = body.into_form().unwrap();
    /// assert_eq!("John", &user.username);
    /// assert_eq!("qwerty", &user.password);
    /// ```
    pub fn into_form<T: DeserializeOwned>(&self) -> crate::Result<T> {
        match &self {
            Body::Some(bytes) => {
                serde_urlencoded::from_bytes::<T>(bytes).or(Err(StatusCode::BadRequest))
            }
            Body::None => Err(StatusCode::InternalServerError),
        }
    }

    /// Parse `Body` as `application/json` data.
    ///
    /// # Examples
    /// ```rust
    /// use qz::body::Body;
    /// use serde::Serialize;
    /// #[derive(Serialize)]
    /// struct User {
    ///     username: String,
    ///     password: String,
    /// }
    ///
    /// let user = User { username: "John".to_string(), password: "qwerty".to_string() };
    /// let body = Body::from_json(&user).unwrap();
    /// assert_eq!(Body::from(r#"{"username":"John","password":"qwerty"}"#), body);
    /// ```
    pub fn from_json(json: &impl Serialize) -> crate::Result<Body> {
        let bytes = serde_json::to_vec(&json).or(Err(StatusCode::BadRequest))?;
        Ok(Body::from(bytes))
    }

    /// Parse `Body` as `application/json` data.
    ///
    /// # Examples
    /// ```rust
    /// use qz::body::Body;
    /// use serde::Deserialize;
    /// #[derive(Deserialize)]
    /// struct User {
    ///     username: String,
    ///     password: String,
    /// }
    ///
    /// let body = Body::from(r#"{"username": "John", "password": "qwerty"}"#);
    /// let user: User = body.into_json().unwrap();
    /// assert_eq!("John", &user.username);
    /// assert_eq!("qwerty", &user.password);
    /// ```
    pub fn into_json<T: DeserializeOwned>(&self) -> crate::Result<T> {
        match &self {
            Body::Some(bytes) => {
                serde_json::from_slice::<T>(&bytes).or(Err(StatusCode::BadRequest))
            }
            Body::None => Err(StatusCode::InternalServerError),
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
