use crate::{
    body::Body,
    header::{HeaderName, HeaderValue},
    mime,
    status::StatusCode,
    Version,
};
use std::collections::HashMap;
use tokio::io::{self, AsyncWrite, AsyncWriteExt};

/// Builder of `Response`.
pub trait Responder: Sized {
    fn respond_to(self) -> Response;
}

impl Responder for () {
    fn respond_to(self) -> Response {
        Response::default()
    }
}

impl Responder for Response {
    fn respond_to(self) -> Response {
        self
    }
}

impl Responder for StatusCode {
    fn respond_to(self) -> Response {
        Response {
            status_code: self,
            version: Version::default(),
            headers: HashMap::default(),
            body: Body::default(),
        }
    }
}

impl Responder for &'static str {
    fn respond_to(self) -> Response {
        Response {
            status_code: StatusCode::Ok,
            version: Version::default(),
            headers: vec![
                (
                    HeaderName::ContentLength,
                    self.len().to_string().as_bytes().to_vec(),
                ),
                (HeaderName::ContentType, mime::TEXT_PLAIN.to_vec()),
            ]
            .into_iter()
            .collect(),
            body: Body::from(self),
        }
    }
}

impl Responder for Vec<u8> {
    fn respond_to(self) -> Response {
        Response {
            status_code: StatusCode::Ok,
            version: Version::default(),
            headers: vec![(
                HeaderName::ContentLength,
                self.len().to_string().as_bytes().to_vec(),
            )]
            .into_iter()
            .collect(),
            body: Body::from(self),
        }
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

    pub fn set_header(&mut self, name: HeaderName, value: HeaderValue) {
        self.headers.insert(name, value);
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn set_body(&mut self, bytes: Vec<u8>) {
        self.body = Body::Some(bytes);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_from_str() {
        let response = "Hello, World!".respond_to();
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
        let response = StatusCode::NotFound.respond_to();
        assert_eq!(StatusCode::NotFound, response.status_code());
        assert_eq!(0, response.headers().len());
        assert_eq!(&Body::None, response.body());
    }
}
