use qz::{request::Request, server::ServerBuilder};
use std::io;

async fn hello(_request: Request) -> &'static str {
    "hello"
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let port = 8080;
    ServerBuilder::new(port)
        .await?
        .route("/", |_| async { "It works!" })
        .serve_dir("/", "./html")
        .route("/hello", hello)
        .build()
        .run()
        .await?;
    Ok(())
}
