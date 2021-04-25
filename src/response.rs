use tokio::io::{self, AsyncWrite, AsyncWriteExt};

use crate::{
    body::Body,
    header::{HeaderName, HeaderValue},
    status::StatusCode,
    Version,
};
use std::collections::HashMap;

/// Builder of `Response`.
pub trait Responder: Sized {
    fn respond_to(self) -> Response;
}

impl Responder for () {
    fn respond_to(self) -> Response {
        Response {
            status_code: StatusCode::Ok,
            version: Version::default(),
            headers: HashMap::default(),
            body: Body::default(),
        }
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
        let mut headers = HashMap::new();
        headers.insert(
            HeaderName::ContentLength,
            self.len().to_string().as_bytes().to_vec(),
        );
        Response {
            status_code: StatusCode::Ok,
            version: Version::default(),
            headers,
            body: Body::from(self),
        }
    }
}

impl Responder for Vec<u8> {
    fn respond_to(self) -> Response {
        let mut headers = HashMap::new();
        headers.insert(
            HeaderName::ContentLength,
            self.len().to_string().as_bytes().to_vec(),
        );
        Response {
            status_code: StatusCode::Ok,
            version: Version::default(),
            headers,
            body: Body::from(self),
        }
    }
}

/// Represents HTTP response.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Response {
    status_code: StatusCode,
    headers: HashMap<HeaderName, HeaderValue>,
    version: Version,
    body: Body,
}

impl Response {
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    pub fn version(&self) -> Version {
        self.version
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
        let mut headers = HashMap::new();
        headers.insert(HeaderName::ContentLength, b"13".to_vec());
        assert_eq!(
            Response {
                status_code: StatusCode::Ok,
                version: Version::OneDotOne,
                headers,
                body: Body::Some(b"Hello, World!".to_vec()),
            },
            "Hello, World!".respond_to()
        );
    }

    #[test]
    fn response_from_status_code() {
        assert_eq!(
            Response {
                status_code: StatusCode::NotFound,
                version: Version::OneDotOne,
                headers: HashMap::new(),
                body: Body::None
            },
            StatusCode::NotFound.respond_to()
        );
    }
}
