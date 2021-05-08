use qz::{method::Method, request::Request, response::Response, server::Server};
use std::io;

async fn hello(_request: Request, _: ()) -> impl Into<Response> {
    "hello"
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let server = Server::builder().route("/", Method::Get, hello).build();
    Server::run(server, 8080).await
}
