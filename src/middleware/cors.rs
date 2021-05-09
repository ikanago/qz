use crate::{
    header::{HeaderName, HeaderValue},
    middleware::{Middleware, MiddlewareChain},
    request::Request,
    response::Response,
    status::StatusCode,
};
use async_trait::async_trait;
use std::convert::From;

/// Middleware to handle CORS issue.
#[derive(Debug)]
pub struct Cors {
    allow_origin: Origin,
}

impl Cors {
    pub fn new() -> Self {
        Self {
            allow_origin: Origin::Any,
        }
    }

    pub fn allow_origin(mut self, origin: impl Into<Origin>) -> Self {
        self.allow_origin = origin.into();
        self
    }

    fn is_valid_origin(&self, origin: &HeaderValue) -> bool {
        match &self.allow_origin {
            Origin::Any => true,
            Origin::Single(v) => v == origin,
        }
    }
}

#[async_trait]
impl<State> Middleware<State> for Cors
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(
        &self,
        request: Request,
        state: State,
        next: MiddlewareChain<'_, State>,
    ) -> Response {
        let origin = match request.get_header(HeaderName::Origin) {
            Some(origin) => origin,
            None => return next.run(request, state).await,
        };
        if !self.is_valid_origin(origin) {
            return Response::new(StatusCode::Unauthorized);
        }

        let mut response = next.run(request, state).await;
        response.set_header(
            HeaderName::AccessControlAllowOrigin,
            self.allow_origin.as_ref(),
        );
        response
    }
}

/// Represents origin. For simplicity, multiple origins are not supported.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Origin {
    /// Wildcard, means any origin.
    Any,
    /// One origin.
    Single(Vec<u8>),
}

impl From<Vec<u8>> for Origin {
    fn from(v: Vec<u8>) -> Self {
        if v == &[b'*'] {
            return Self::Any;
        }
        Self::Single(v)
    }
}

impl From<&Vec<u8>> for Origin {
    fn from(v: &Vec<u8>) -> Self {
        Self::from(v.clone())
    }
}

impl From<&[u8]> for Origin {
    fn from(v: &[u8]) -> Self {
        Self::from(v.to_vec())
    }
}

impl From<String> for Origin {
    fn from(s: String) -> Self {
        if s == "*" {
            return Self::Any;
        }
        Self::Single(s.into_bytes())
    }
}

impl From<&str> for Origin {
    fn from(s: &str) -> Self {
        Self::from(s.to_string())
    }
}

impl AsRef<[u8]> for Origin {
    fn as_ref(&self) -> &[u8] {
        match self {
            Origin::Any => b"*",
            Origin::Single(v) => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{method::Method, server::ServerBuilder};

    use super::*;

    const ALLOW_ORIGIN: &[u8] = b"localhost:3000";

    fn server() -> ServerBuilder<()> {
        ServerBuilder::new().route("/", Method::Get, |_, _| async { "Hello" })
    }

    // Simple request is a request which does not need a preflight request.
    #[tokio::test]
    async fn simple_request_any_origin() {
        let server = server().with(Cors::new()).build();
        let request = Request::builder()
            .set_header(HeaderName::Origin, ALLOW_ORIGIN)
            .build();
        let response = server.respond(request).await;
        assert_eq!(StatusCode::Ok, response.status_code());
        assert_eq!(
            Origin::Any,
            response
                .get_header(&HeaderName::AccessControlAllowOrigin)
                .unwrap()
                .clone()
                .into()
        );
    }

    #[tokio::test]
    async fn simple_request_specific_origin() {
        let server = server()
            .with(Cors::new().allow_origin(ALLOW_ORIGIN))
            .build();
        let request = Request::builder()
            .set_header(HeaderName::Origin, ALLOW_ORIGIN)
            .build();
        let response = server.respond(request).await;
        assert_eq!(StatusCode::Ok, response.status_code());
        assert_eq!(
            Origin::Single(ALLOW_ORIGIN.to_vec()),
            response
                .get_header(&HeaderName::AccessControlAllowOrigin)
                .unwrap()
                .clone()
                .into()
        );
    }

    #[tokio::test]
    async fn simple_request_without_origin_header() {
        let server = server()
            .with(Cors::new().allow_origin(ALLOW_ORIGIN))
            .build();
        let request = Request::default();
        let response = server.respond(request).await;
        assert_eq!(StatusCode::Ok, response.status_code());
    }
}
