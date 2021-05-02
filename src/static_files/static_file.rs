use std::{
    io,
    path::{Path, PathBuf},
};

use crate::{handler::Handler, mime, request::Request, response::Response};
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
    async fn call(&self, _request: Request) -> crate::Result<Response> {
        let filename = self.path.clone();
        let mime_type = mime::filename_to_mime(&filename);
        // This should return internal server error.
        let mut file_to_serve = File::open(filename).await?;

        let mut buffer = Vec::new();
        file_to_serve.read_to_end(&mut buffer).await?;
        let mut response = Response::from(buffer);
        response.set_content_type(mime_type);
        Ok(response)
    }
}
