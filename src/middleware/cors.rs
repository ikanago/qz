use crate::{
    header::{HeaderName, HeaderValue},
    method::Method,
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
    allow_methods: HeaderValue,
    allow_headers: HeaderValue,
    max_age: HeaderValue,
}

const DEFAULT_ALLOW_METHODS: &[u8] = b"POST, GET, OPTIONS";
const DEFAULT_ALLOW_HEADERS: &[u8] = b"*";
const DEFAULT_MAX_AGE: &[u8] = b"86400";

impl Cors {
    pub fn new() -> Self {
        Self {
            allow_origin: Origin::Any,
            allow_methods: DEFAULT_ALLOW_METHODS.into(),
            allow_headers: DEFAULT_ALLOW_HEADERS.into(),
            max_age: DEFAULT_MAX_AGE.into(),
        }
    }

    pub fn allow_origin(mut self, origin: impl Into<Origin>) -> Self {
        self.allow_origin = origin.into();
        self
    }

    pub fn allow_methods(mut self, methods: impl Into<HeaderValue>) -> Self {
        self.allow_methods = methods.into();
        self
    }

    pub fn allow_headers(mut self, headers: impl Into<HeaderValue>) -> Self {
        self.allow_headers = headers.into();
        self
    }

    pub fn max_age(mut self, max_age: impl Into<HeaderValue>) -> Self {
        self.max_age = max_age.into();
        self
    }

    fn is_valid_origin(&self, origin: &HeaderValue) -> bool {
        match &self.allow_origin {
            Origin::Any => true,
            Origin::Single(v) => v == origin,
        }
    }

    fn handle_preflight(&self) -> Response {
        let mut response = Response::new(StatusCode::Ok);
        response.set_header(
            HeaderName::AccessControlAllowOrigin,
            self.allow_origin.as_ref(),
        );
        response.set_header(
            HeaderName::AccessControlAllowMethods,
            self.allow_methods.clone(),
        );
        response.set_header(
            HeaderName::AccessControlAllowHeaders,
            self.allow_headers.clone(),
        );
        response.set_header(HeaderName::AccessControlAllowMaxAge, self.max_age.clone());
        response
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

        if request.method() == Method::Options {
            return self.handle_preflight();
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
    use crate::server::ServerBuilder;

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

    #[tokio::test]
    async fn handle_preflight_request() {
        let server = server()
            .with(Cors::new().allow_origin(ALLOW_ORIGIN))
            .build();
        let request = Request::builder()
            .set_method(Method::Options)
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
        assert_eq!(
            HeaderValue::from(DEFAULT_ALLOW_METHODS),
            response
                .get_header(&HeaderName::AccessControlAllowMethods)
                .unwrap()
                .clone()
        );
        assert_eq!(
            HeaderValue::from(DEFAULT_ALLOW_HEADERS),
            response
                .get_header(&HeaderName::AccessControlAllowHeaders)
                .unwrap()
                .clone()
        );
        assert_eq!(
            HeaderValue::from(DEFAULT_MAX_AGE),
            response
                .get_header(&HeaderName::AccessControlAllowMaxAge)
                .unwrap()
                .clone()
        );
    }
}
