use tokio::io::{self, AsyncWrite, AsyncWriteExt};

use crate::{status::StatusCode, Version};
use std::convert::From;

/// Represents HTTP response.
#[derive(Debug, Default)]
pub struct Response {
    status_code: StatusCode,
    version: Version,
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
        connection.write_all(b"\r\n\r\n").await?;
        connection.flush().await
    }
}

impl From<StatusCode> for Response {
    fn from(code: StatusCode) -> Self {
        Self {
            status_code: code,
            version: Version::default(),
        }
    }
}
