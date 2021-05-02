use crate::{
    body::Body,
    header::{HeaderName, HeaderValue},
    mime,
    status::StatusCode,
    Version,
};
use std::{collections::HashMap, convert::From};
use tokio::io::{self, AsyncWrite, AsyncWriteExt};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ResponseBuilder {
    inner: Response,
}

impl ResponseBuilder {
    pub fn new() -> Self {
        Self {
            inner: Response::default(),
        }
    }

    pub fn set_status_code(mut self, status_code: StatusCode) -> Self {
        self.inner.status_code = status_code;
        self
    }

    pub fn set_header(mut self, name: HeaderName, value: impl Into<HeaderValue>) -> Self {
        self.inner.set_header(name, value.into());
        self
    }

    pub fn set_body(mut self, body: impl Into<Body>) -> Self {
        self.inner.set_body(body);
        self
    }

    pub fn build(self) -> Response {
        self.inner
    }
}

/// Represents HTTP response.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Response {
    pub(crate) status_code: StatusCode,
    pub(crate) headers: HashMap<HeaderName, HeaderValue>,
    pub(crate) version: Version,
    pub(crate) body: Body,
}

impl Response {
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder::default()
    }

    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn headers(&self) -> &HashMap<HeaderName, HeaderValue> {
        &self.headers
    }

    pub fn get_header(&self, name: &HeaderName) -> Option<&HeaderValue> {
        self.headers.get(name)
    }

    pub fn set_header(&mut self, name: HeaderName, value: impl Into<HeaderValue>) {
        self.headers.insert(name, value.into());
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn set_body(&mut self, bytes: impl Into<Body>) {
        self.body = bytes.into()
    }

    pub async fn send<W>(&self, connection: &mut W) -> io::Result<()>
    where
        W: AsyncWrite + Unpin,
    {
        connection.write_all(b"HTTP/").await?;
        connection.write_all(self.version.as_bytes()).await?;
        connection.write_all(b" ").await?;
        connection.write_all(&self.status_code.as_bytes()).await?;
        connection.write_all(b" ").await?;
        connection
            .write_all(self.status_code.reason_phrase())
            .await?;
        connection.write_all(b"\r\n").await?;
        for (name, value) in self.headers.iter() {
            // Consider to use `AsyncWriteExt::write_vectored()`
            connection.write_all(name.as_ref()).await?;
            connection.write_all(b": ").await?;
            connection.write_all(&value).await?;
            connection.write_all(b"\r\n").await?;
        }
        connection.write_all(b"\r\n").await?;
        connection.write_all(self.body.as_ref()).await?;
        connection.flush().await
    }
}

impl From<StatusCode> for Response {
    fn from(code: StatusCode) -> Self {
        Response::builder().set_status_code(code).build()
    }
}

impl<'a> From<&'a str> for Response {
    fn from(s: &'a str) -> Self {
        Response::builder()
            .set_header(HeaderName::ContentLength, s.len().to_string())
            .set_header(HeaderName::ContentType, mime::TEXT_PLAIN.to_vec())
            .set_body(Body::from(s))
            .build()
    }
}

impl From<Vec<u8>> for Response {
    fn from(bytes: Vec<u8>) -> Self {
        Response::builder()
            .set_header(HeaderName::ContentLength, bytes.len().to_string())
            .set_body(Body::from(bytes))
            .build()
    }
}

impl<'a> From<&'a [u8]> for Response {
    fn from(bytes: &'a [u8]) -> Self {
        Response::builder()
            .set_header(HeaderName::ContentLength, bytes.len().to_string())
            .set_body(Body::from(bytes))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_from_str() {
        let response = Response::from("Hello, World!");
        assert_eq!(StatusCode::Ok, response.status_code());
        assert_eq!(
            Some(&b"13".to_vec()),
            response.get_header(&HeaderName::ContentLength)
        );
        assert_eq!(
            Some(&b"text/plain".to_vec()),
            response.get_header(&HeaderName::ContentType)
        );
        assert_eq!(&Body::Some(b"Hello, World!".to_vec()), response.body());
    }

    #[test]
    fn response_from_status_code() {
        let response = Response::from(StatusCode::NotFound);
        assert_eq!(StatusCode::NotFound, response.status_code());
        assert_eq!(0, response.headers().len());
        assert_eq!(&Body::None, response.body());
    }
}
