use crate::{request::Request, response::Response};
use async_trait::async_trait;
use std::{fmt, future::Future};

/// Abstruction over all process to create response from request.
///
/// `State` is something to use in handler process such as database connection, counter and so on.
#[async_trait]
pub trait Handler<State>: Send + Sync + 'static
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(&self, request: Request, state: State) -> crate::Result<Response>;
}

#[async_trait]
impl<State, F, Fut> Handler<State> for F
where
    State: Clone + Send + Sync + 'static,
    F: Send + Sync + 'static + Fn(Request, State) -> Fut,
    Fut: Future + Send + 'static,
    Fut::Output: Into<Response>,
{
    async fn call(&self, request: Request, state: State) -> crate::Result<Response> {
        Ok(self(request, state).await.into())
    }
}

impl<State> fmt::Debug for dyn Handler<State>
where
    State: Clone + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handler")
    }
}
