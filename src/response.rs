use crate::{status::StatusCode, Version};
use std::convert::From;

#[derive(Debug, Default)]
pub struct Response {
    status_code: StatusCode,
    version: Version,
}

impl Response {
    pub fn into_bytes(&self) -> String {
        format!(
            "HTTP/{} {} {}\r\n\r\n",
            self.version.as_str(),
            self.status_code.as_str(),
            self.status_code.reason_phrase()
        )
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
