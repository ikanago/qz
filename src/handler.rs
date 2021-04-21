use crate::{request::Request, response::Response};
use std::fmt;

pub trait Handler: Send + Sync + 'static {
    fn call(&self, request: Request) -> Response;
}

impl<F: Send + Sync + 'static> Handler for F
where
    F: Fn(Request) -> Response,
{
    fn call(&self, request: Request) -> Response {
        self(request)
    }
}

impl fmt::Debug for dyn Handler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Handler")
    }
}
