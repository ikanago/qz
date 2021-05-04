use qz::{method::Method, redirect::Redirect, server::ServerBuilder};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    ServerBuilder::new(8080)
        .await?
        .route("/hello", Method::Get, |_| async { "Hello" })
        .route(
            "/obsolete",
            Method::Get,
            Redirect::moved_permanently("/hello"),
        )
        .route("/maintainance", Method::Get, Redirect::found("/hello"))
        .route("/login", Method::Post, Redirect::see_other("/hello"))
        .build()
        .run()
        .await
}
