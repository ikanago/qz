use qz::{method::Method, middleware::Cors, server::Server};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let server = Server::builder()
        .with(Cors::new().allow_origin("http://localhost:3000"))
        .route("/", Method::Get, |_, _| async { "Hello" })
        .build();
    Server::run(server, 8080).await
}
