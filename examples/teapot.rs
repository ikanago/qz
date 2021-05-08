use qz::{
    method::Method, request::Request, response::Response, server::Server, status::StatusCode,
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
    let server = Server::builder().route("/", Method::Get, teapot).build();
    Server::run(server, 8080).await
}
