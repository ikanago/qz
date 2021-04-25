use std::{
    io,
    path::{Path, PathBuf},
};

use crate::{
    handler::Handler,
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
        // This should return internal server error.
        let mut file_to_serve = File::open(self.path.clone()).await.unwrap();
        let mut buffer = Vec::new();
        if file_to_serve.read_to_end(&mut buffer).await.is_err() {
            return StatusCode::NotFound.respond_to();
        }
        buffer.respond_to()
    }
}
