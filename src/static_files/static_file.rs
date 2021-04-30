use std::{
    io,
    path::{Path, PathBuf},
};

use crate::{
    handler::Handler,
    header::HeaderName,
    mime,
    request::Request,
    response::{Responder, Response},
    status::StatusCode,
};
use async_trait::async_trait;
use tokio::{fs::File, io::AsyncReadExt};

pub struct StaticFile {
    path: PathBuf,
}

impl StaticFile {
    pub fn mount<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref().canonicalize()?;
        Ok(Self { path })
    }
}

#[async_trait]
impl Handler for StaticFile {
    async fn call(&self, _request: Request) -> Response {
        let filename = self.path.clone();
        let mime_type = mime::filename_to_mime(&filename);
        // This should return internal server error.
        let mut file_to_serve = File::open(filename).await.unwrap();

        let mut buffer = Vec::new();
        if file_to_serve.read_to_end(&mut buffer).await.is_err() {
            return StatusCode::NotFound.respond_to();
        }
        let mut response = buffer.respond_to();
        response.set_header(HeaderName::ContentType, mime_type.to_vec());
        response
    }
}
