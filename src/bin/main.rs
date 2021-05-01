use qz::{middleware::BasicAuth, redirect::Redirect, request::Request, server::ServerBuilder, Uri};
use std::io;

async fn hello(_request: Request) -> &'static str {
    "hello"
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let port = 8080;
    ServerBuilder::new(port)
        .await?
        .with(BasicAuth::new("user", "password", Uri::from("/hello")))
        .route("/", |_| async { "It works!" })
        // .serve_dir("/", "./html")
        .route("/hello", hello)
        .route("/example", Redirect::new(Uri::from("http://example.com")))
        .build()
        .run()
        .await?;
    Ok(())
}
