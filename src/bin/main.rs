extern crate qz;

use qz::{request::Request, response::Responder, server::ServerBuilder, status::StatusCode};

async fn hello(_request: Request) -> &'static str {
    "hello"
}

async fn hoge(_request: Request) -> impl Responder {
    StatusCode::Ok
}

#[tokio::main]
async fn main() {
    let port = 8080;
    let server = ServerBuilder::new(port)
        .await
        .unwrap()
        .route("/", hello)
        .route("/hoge", hoge)
        .route("/fuga", |_| async { "fuga" })
        .build();
    server.run().await.unwrap();
}
