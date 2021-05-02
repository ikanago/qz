use std::path::{Path, PathBuf};

use crate::{
    handler::Handler, mime, request::Request, response::Response, static_files::find_file,
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
    async fn call(&self, request: Request) -> crate::Result<Response> {
        let found_file = find_file(
            request.uri(),
            self.mount_dir.as_path(),
            self.serve_at.as_path(),
        )?;

        let mime_type = mime::filename_to_mime(&found_file);
        let mut file_to_serve = File::open(found_file).await?;
        let mut buffer = Vec::new();
        file_to_serve.read_to_end(&mut buffer).await?;
        let mut response = Response::from(buffer);
        response.set_content_type(mime_type);
        Ok(response)
    }
}
