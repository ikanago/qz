use async_trait::async_trait;

use crate::{request::Request, response::Response};
use std::future::Future;

mod basic_auth;

pub use basic_auth::BasicAuth;

/// Middleware preprocesses request before generating response in `Handler`.
/// Some process need to return not just status code but response object in failure.
#[async_trait]
pub trait Middleware: Send + Sync + 'static {
    async fn call(&self, request: Request) -> crate::Result<Request, Response>;
}

#[async_trait]
impl<F, Fut> Middleware for F
where
    F: Send + Sync + 'static + Fn(Request) -> Fut,
    Fut: Future<Output = crate::Result<Request, Response>> + Send + 'static,
{
    async fn call(&self, request: Request) -> crate::Result<Request, Response> {
        self(request).await
    }
}

/// Holds array of middlewares.
/// Apply middlewares in the order of registration.
pub struct MiddlewareChain {
    middlewares: Vec<Box<dyn Middleware>>,
}

impl MiddlewareChain {
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }

    pub fn push<M: Middleware>(&mut self, middleware: M) {
        self.middlewares.push(Box::new(middleware));
    }

    pub async fn run(&self, mut request: Request) -> crate::Result<Request, Response> {
        // Cannot use `try_fold()` bacause it needs non-async closure.
        for middleware in self.middlewares.iter() {
            request = middleware.call(request).await?;
        }
        Ok(request)
    }
}
