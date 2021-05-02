use qz::{
    body::Body, method::Method, middleware::BasicAuth, redirect::Redirect, request::Request,
    response::Response, server::ServerBuilder, status::StatusCode,
};
use std::io;

async fn teapot(_request: Request) -> Response {
    Response::builder()
        .set_status_code(StatusCode::ImaTeapot)
        .set_body(b"<h1>I'm a teapot;)</h1>".to_vec())
        .build()
}

async fn echo(request: Request) -> Body {
    request.body().clone()
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let port = 8080;
    ServerBuilder::new(port)
        .await?
        .with(BasicAuth::new("user", "password", "/hello"))
        .route("/", Method::Get, |_| async { "It works!" })
        // .serve_dir("/", "./html")
        .route("/teapot", Method::Get, teapot)
        .route("/echo", Method::Post, echo)
        .route("/example", Method::Get, Redirect::new("http://example.com"))
        .build()
        .run()
        .await?;
    Ok(())
}
