use async_trait::async_trait;

use crate::{handler::Handler, request::Request, response::Response};
use std::{future::Future, sync::Arc};

mod basic_auth;
mod cors;

pub use basic_auth::BasicAuth;

/// Middleware preprocesses request before generating response in `Handler` and postprocesses
/// response.
#[async_trait]
pub trait Middleware<State>: Send + Sync + 'static
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(
        &self,
        request: Request,
        state: State,
        next: MiddlewareChain<'_, State>,
    ) -> Response;
}

#[async_trait]
impl<F, Fut, State> Middleware<State> for F
where
    F: Send + Sync + 'static + Fn(Request, State, MiddlewareChain<'_, State>) -> Fut,
    Fut: Future<Output = Response> + Send + 'static,
    State: Clone + Send + Sync + 'static,
{
    async fn call(
        &self,
        request: Request,
        state: State,
        next: MiddlewareChain<'_, State>,
    ) -> Response {
        self(request, state, next).await
    }
}

/// Sequence of Middlewares.
///
/// Middlewares' process is recursive:
/// * Modify `Request` (basically modify headers)
/// * Run next middleware
/// * Modify `Response` (basically modify headers)
pub struct MiddlewareChain<'a, State>
where
    State: Clone + Send + Sync + 'static,
{
    pub(crate) handler: &'a dyn Handler<State>,
    pub(crate) middlewares: &'a [Arc<dyn Middleware<State>>],
}

impl<State> MiddlewareChain<'_, State>
where
    State: Clone + Send + Sync + 'static,
{
    pub async fn run(mut self, request: Request, state: State) -> Response {
        if let Some((first, remaining)) = self.middlewares.split_first() {
            self.middlewares = remaining;
            first.call(request, state, self).await
        } else {
            match self.handler.call(request, state).await {
                Ok(response) => response,
                Err(code) => code.into(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        body::Body,
        header::{HeaderName, HeaderValue},
        method::Method,
        server::ServerBuilder,
    };

    use super::*;

    #[tokio::test]
    async fn middleware_chain_handler_only() {
        let handler = Box::new(|_, _| async { "hello" });
        let chain = MiddlewareChain {
            handler: handler.as_ref(),
            middlewares: &Vec::new(),
        };
        let response = chain.run(Request::default(), ()).await;
        assert_eq!(&Body::from("hello"), response.body());
    }

    #[tokio::test]
    async fn middleware_chain() {
        struct DummyA;
        #[async_trait]
        impl Middleware<()> for DummyA {
            async fn call(
                &self,
                mut request: Request,
                state: (),
                next: MiddlewareChain<'_, ()>,
            ) -> Response {
                // These headers here has no meaning.
                request.set_header(HeaderName::Accept, "*/*");
                let mut response = next.run(request, state).await;
                response.set_header(HeaderName::Unknown, "hello");
                response
            }
        }

        struct DummyB;
        #[async_trait]
        impl Middleware<()> for DummyB {
            async fn call(
                &self,
                mut request: Request,
                state: (),
                next: MiddlewareChain<'_, ()>,
            ) -> Response {
                request.set_header(HeaderName::Host, "localhost");
                let mut response = next.run(request, state).await;
                response.set_header(HeaderName::Location, "example.com");
                response
            }
        }

        struct Test;
        #[async_trait]
        impl Middleware<()> for Test {
            async fn call(
                &self,
                request: Request,
                state: (),
                next: MiddlewareChain<'_, ()>,
            ) -> Response {
                assert_eq!(
                    Some(&HeaderValue::from("*/*")),
                    request.get_header(HeaderName::Accept)
                );
                assert_eq!(
                    Some(&HeaderValue::from("localhost")),
                    request.get_header(HeaderName::Host)
                );
                next.run(request, state).await
            }
        }

        let server = ServerBuilder::new()
            .route("/", Method::Get, Box::new(|_, _| async { "hello" }))
            .with(DummyA)
            .with(DummyB)
            .with(Test)
            .build();
        let request = Request::default();
        let response = server.respond(request).await;
        assert_eq!(
            Some(&HeaderValue::from("hello")),
            response.get_header(&HeaderName::Unknown)
        );
        assert_eq!(
            Some(&HeaderValue::from("example.com")),
            response.get_header(&HeaderName::Location)
        );
    }
}
