use qz::{method::Method, middleware::BasicAuth, server::Server};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let server = Server::builder()
        .with(BasicAuth::new("user", "password", "/"))
        .route("/", Method::Get, |_, _| async { "Hello" })
        .build();
    Server::run(server, 8080).await
}
