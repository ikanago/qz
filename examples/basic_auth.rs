use qz::{method::Method, middleware::BasicAuth, server::ServerBuilder};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    ServerBuilder::new(8080)
        .await?
        .with(BasicAuth::new("user", "password", "/"))
        .route("/", Method::Get, |_, _| async { "Hello" })
        .build()
        .run()
        .await
}
