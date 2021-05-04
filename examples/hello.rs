use qz::{method::Method, request::Request, response::Response, server::ServerBuilder};
use std::io;

async fn hello(_request: Request) -> impl Into<Response> {
    "hello"
}

#[tokio::main]
async fn main() -> io::Result<()> {
    ServerBuilder::new(8080)
        .await?
        .route("/", Method::Get, hello)
        .build()
        .run()
        .await
}
