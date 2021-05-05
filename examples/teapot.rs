use qz::{
    method::Method, request::Request, response::Response, server::ServerBuilder, status::StatusCode,
};
use std::io;

async fn teapot(_request: Request, _: ()) -> impl Into<Response> {
    Response::builder()
        .set_status_code(StatusCode::ImaTeapot)
        .set_body("<h1>I'm a Teapot;)</h1>")
        .build()
}

#[tokio::main]
async fn main() -> io::Result<()> {
    ServerBuilder::new(8080)
        .await?
        .route("/", Method::Get, teapot)
        .build()
        .run()
        .await
}
