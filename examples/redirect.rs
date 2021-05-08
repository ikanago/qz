use qz::{method::Method, redirect::Redirect, server::Server};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let server = Server::builder()
        .route("/hello", Method::Get, |_, _| async { "Hello" })
        .route(
            "/obsolete",
            Method::Get,
            Redirect::moved_permanently("/hello"),
        )
        .route("/maintainance", Method::Get, Redirect::found("/hello"))
        .route("/login", Method::Post, Redirect::see_other("/hello"))
        .build();
    Server::run(server, 8080).await
}
