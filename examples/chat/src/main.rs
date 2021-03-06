mod db;
mod model;

use std::{
    io,
    sync::{Arc, RwLock},
};

use db::Db;
use model::{Post, User};
use qz::{
    body::Body, method::Method, middleware::Cors, mime, redirect::Redirect, request::Request,
    response::Response, server::Server, status::StatusCode,
};

async fn register(request: Request, db: Arc<RwLock<Db>>) -> qz::Result<Response> {
    let user: User = request.body_form()?;
    let mut db = db
        .as_ref()
        .write()
        .or(Err(StatusCode::InternalServerError))?;
    db.register(user);
    Ok(Redirect::see_other("/posts").into())
}

async fn posts(_request: Request, db: Arc<RwLock<Db>>) -> qz::Result<Response> {
    let db = db
        .as_ref()
        .read()
        .or(Err(StatusCode::InternalServerError))?;
    let posts = db.current_posts(10);
    let body = Body::from_json(&posts)?;
    Ok(Response::builder()
        .set_body(body)
        .set_content_type(mime::APPLICATION_JSON)
        .build())
}

async fn create_post(request: Request, db: Arc<RwLock<Db>>) -> qz::Result<Response> {
    let post: Post = request.body_json()?;
    let mut db = db
        .as_ref()
        .write()
        .or(Err(StatusCode::InternalServerError))?;
    db.create_post(post);
    Ok(StatusCode::Ok.into())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let db = Arc::new(RwLock::new(Db::new()));

    let server = Server::builder_with_state(db.clone())
        .with(Cors::new())
        .serve_dir("/site", "./frontend/build")
        .route("/register", Method::Post, register)
        .route("/posts", Method::Get, posts)
        .route("/create_post", Method::Post, create_post)
        .build();
    Server::run(server, 8080).await
}
