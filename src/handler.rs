use crate::{request::Request, response::Response};
use async_trait::async_trait;
use std::{fmt, future::Future};

/// Abstruction over all process to create response from request.
#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn call(&self, request: Request) -> crate::Result<Response>;
}

#[async_trait]
impl<F, Fut> Handler for F
where
    F: Send + Sync + 'static + Fn(Request) -> Fut,
    Fut: Future + Send + 'static,
    Fut::Output: Into<Response>,
{
    async fn call(&self, request: Request) -> crate::Result<Response> {
        Ok(self(request).await.into())
    }
}

impl fmt::Debug for dyn Handler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handler")
    }
}
