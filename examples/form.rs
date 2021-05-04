use qz::{
    method::Method, redirect::Redirect, request::Request, response::Response, server::ServerBuilder,
};
use serde::Deserialize;
use std::io;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
struct User {
    username: String,
    password: String,
}

async fn login(request: Request) -> qz::Result<Response> {
    let user: User = request.body_form()?;
    println!("{:?}", user);
    Ok(Redirect::see_other("/userpage").into())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    ServerBuilder::new(8080)
        .await?
        .serve_file("/userpage", "./examples/assets/index.html")?
        .serve_file("/login", "./examples/assets/form.html")?
        .route("/login", Method::Post, login)
        .build()
        .run()
        .await
}
