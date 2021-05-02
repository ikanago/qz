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
    pub fn new(uri: Uri) -> Self {
        Self::found(uri)
    }

    /// Create a permanent redirect to `uri`.
    pub fn moved_permanently(uri: Uri) -> Self {
        Self {
            status_code: StatusCode::MovedPermanently,
            uri,
        }
    }

    /// Create a redirect to `uri`.
    pub fn found(uri: Uri) -> Self {
        Self {
            status_code: StatusCode::Found,
            uri,
        }
    }

    /// Create a see other redirect to `uri`.
    pub fn see_other(uri: Uri) -> Self {
        Self {
            status_code: StatusCode::SeeOther,
            uri,
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
