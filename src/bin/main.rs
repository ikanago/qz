extern crate qz;

use qz::{request::Request, response::Response, server::ServerBuilder, status::StatusCode};

fn hello(_request: Request) -> Response {
    Response::from(StatusCode::Ok)
}

#[tokio::main]
async fn main() {
    let port = 8080;
    let server = ServerBuilder::new(port)
        .await
        .unwrap()
        .route("/", hello)
        .route("/hoge", |_req| Response::from(StatusCode::Ok))
        .build();
    server.run().await.unwrap();
}
