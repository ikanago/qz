use crate::{
    handler::Handler, header::HeaderName, request::Request, response::Response, status::StatusCode,
    Uri,
};
use async_trait::async_trait;

/// Handler performing redirection.
#[derive(Debug)]
pub struct Redirect {
    status_code: StatusCode,
    uri: Uri,
}

impl Redirect {
    /// Create a redirect to `uri`.
    /// This method is alias to `Redirect::found()`.
    pub fn new(uri: impl Into<Uri>) -> Self {
        Self::found(uri.into())
    }

    /// Create a permanent redirect to `uri`.
    pub fn moved_permanently(uri: impl Into<Uri>) -> Self {
        Self {
            status_code: StatusCode::MovedPermanently,
            uri: uri.into(),
        }
    }

    /// Create a redirect to `uri`.
    pub fn found(uri: impl Into<Uri>) -> Self {
        Self {
            status_code: StatusCode::Found,
            uri: uri.into(),
        }
    }

    /// Create a see other redirect to `uri`.
    pub fn see_other(uri: impl Into<Uri>) -> Self {
        Self {
            status_code: StatusCode::SeeOther,
            uri: uri.into(),
        }
    }
}

#[async_trait]
impl Handler for Redirect {
    async fn call(&self, _request: Request) -> crate::Result<Response> {
        let response = Response::builder()
            .set_status_code(self.status_code)
            .set_header(HeaderName::Location, self.uri.0.clone())
            .build();
        Ok(response)
    }
}
