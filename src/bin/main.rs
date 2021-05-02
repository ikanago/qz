use qz::{
    middleware::BasicAuth, redirect::Redirect, request::Request, responder::Responder,
    response::Response, server::ServerBuilder, status::StatusCode, Uri,
};
use std::io;

async fn teapot(_request: Request) -> Response {
    let mut response = StatusCode::IMATeapot.respond_to();
    response.set_body(b"<h1>I'm a teapot;)</h1>".to_vec());
    response
}

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
        .route("/teapot", teapot)
        .route("/hello", hello)
        .route("/example", Redirect::new(Uri::from("http://example.com")))
        .build()
        .run()
        .await?;
    Ok(())
}
