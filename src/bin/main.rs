use qz::{request::Request, server::ServerBuilder};

async fn hello(_request: Request) -> &'static str {
    "hello"
}

#[tokio::main]
async fn main() {
    let port = 8080;
    let server = ServerBuilder::new(port)
        .await
        .unwrap()
        .route("/", hello)
        .serve_dir("./static", "/static")
        .route("/fuga", |_| async { "fuga" })
        .build();
    server.run().await.unwrap();
}
