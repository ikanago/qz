use crate::{request::Request, response::Response};

pub trait Handler: 'static {}

impl<F> Handler for F
where
    F: FnOnce(Request) -> Response + 'static
{}

