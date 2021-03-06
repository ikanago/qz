use qz::{method::Method, request::Request, server::Server};
use std::{
    io,
    sync::{atomic::AtomicUsize, Arc},
};

#[derive(Clone)]
struct Counter {
    value: Arc<AtomicUsize>,
}

impl Counter {
    fn new() -> Self {
        Self {
            value: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn increment(&self) -> usize {
        self.value
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            + 1
    }
}

async fn increment(_request: Request, state: Counter) -> String {
    let value = state.increment();
    value.to_string()
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let server = Server::builder_with_state(Counter::new())
        .route("/increment", Method::Post, increment)
        .build();
    Server::run(server, 8080).await
}
