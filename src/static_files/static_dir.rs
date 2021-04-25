use std::path::{Path, PathBuf};

use crate::{
    handler::Handler,
    request::Request,
    response::{Responder, Response},
    static_files::find_file,
    status::StatusCode,
};
use async_trait::async_trait;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug)]
pub struct StaticDir {
    mount_dir: PathBuf,
    serve_at: PathBuf,
}

impl StaticDir {
    pub fn mount<P1, P2>(mount_dir: P1, serve_at: P2) -> Self
    where
        P1: AsRef<Path>,
        P2: AsRef<Path>,
    {
        Self {
            mount_dir: mount_dir.as_ref().to_path_buf(),
            serve_at: serve_at.as_ref().to_path_buf(),
        }
    }
}

#[async_trait]
impl Handler for StaticDir {
    async fn call(&self, request: Request) -> Response {
        let found_file = match find_file(
            request.uri(),
            self.mount_dir.as_path(),
            self.serve_at.as_path(),
        ) {
            Ok(path) => path,
            Err(()) => return StatusCode::NotFound.respond_to(),
        };

        let mut file_to_serve = match File::open(found_file).await {
            Ok(file) => file,
            Err(_) => return StatusCode::NotFound.respond_to(),
        };
        let mut buffer = Vec::new();
        if let Err(_) = file_to_serve.read_to_end(&mut buffer).await {
            return StatusCode::NotFound.respond_to();
        }
        buffer.respond_to()
    }
}
