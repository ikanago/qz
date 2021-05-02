use qz::{
    middleware::BasicAuth, redirect::Redirect, request::Request, response::Response,
    server::ServerBuilder, status::StatusCode,
};
use std::io;

async fn teapot(_request: Request) -> Response {
    Response::builder()
        .set_status_code(StatusCode::IMATeapot)
        .set_body(b"<h1>I'm a teapot;)</h1>".to_vec())
        .build()
}

async fn hello(_request: Request) -> &'static str {
    "hello"
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let port = 8080;
    ServerBuilder::new(port)
        .await?
        .with(BasicAuth::new("user", "password", "/hello"))
        .route("/", |_| async { "It works!" })
        // .serve_dir("/", "./html")
        .route("/teapot", teapot)
        .route("/hello", hello)
        .route("/example", Redirect::new("http://example.com"))
        .build()
        .run()
        .await?;
    Ok(())
}
