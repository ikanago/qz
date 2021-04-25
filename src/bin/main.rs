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
        // .route("/", |_| async { "It works!" })
        .serve_dir("/", "./html")
        .serve_dir("/static", "./static")
        .route("/hello", hello)
        .route("/fuga", |_| async { "fuga" })
        .build()
        .run()
        .await?;
    Ok(())
}
