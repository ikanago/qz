use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::{
    handler::Handler,
    request::Request,
    response::{Responder, Response},
    status::StatusCode,
    Uri,
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

    fn find_file(&self, path: &Uri) -> Result<PathBuf, ()> {
        let path = std::str::from_utf8(&path.0).unwrap();
        let path = Path::new(path);
        let path = match path.strip_prefix(self.serve_at.as_path()) {
            Ok(path) => path,
            Err(_) => return Err(()),
        };

        let mut file_to_find = self.mount_dir.clone();
        for p in path {
            if p == OsStr::new(".") {
                continue;
            } else if p == OsStr::new("..") {
                if !file_to_find.pop() {
                    // Forbid to access file which is out of mount point to prevent directory
                    // traversal attack.
                    return Err(());
                }
            } else {
                file_to_find.push(p);
            }
        }

        if !file_to_find.starts_with(&self.mount_dir) {
            return Err(());
        }
        if !file_to_find.exists() {
            return Err(());
        }
        Ok(file_to_find)
    }
}

#[async_trait]
impl Handler for StaticDir {
    async fn call(&self, request: Request) -> Response {
        let found_file = match self.find_file(request.uri()) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::{fs, io};

    // Creates ./static_dir_test/static/index.html for tests.
    // First this function uses `tempfile` crate, but temporary directory which is created with the
    // crate is deleted when it goes out of scope.
    async fn setup_dir() -> io::Result<StaticDir> {
        let static_dir = PathBuf::from("./static_dir_test/static");
        fs::create_dir_all(static_dir.clone()).await?;

        let file_path = static_dir.join("index.html");
        File::create(file_path).await?;
        Ok(StaticDir::mount(static_dir, "/static"))
    }

    #[tokio::test]
    async fn find_file() -> io::Result<()> {
        let static_dir = setup_dir().await?;
        static_dir
            .find_file(&Uri::new(b"/static/index.html"))
            .unwrap();
        Ok(())
    }

    #[tokio::test]
    async fn find_missing_file() -> io::Result<()> {
        let static_dir = setup_dir().await?;
        assert!(static_dir
            .find_file(&Uri::new(b"/static/index.css"))
            .is_err());
        Ok(())
    }

    #[tokio::test]
    async fn invalid_access() -> io::Result<()> {
        let static_dir = setup_dir().await?;
        File::create("./static_dir_test/secret.txt").await?;
        /*
         * /static_dir_test
         * `-/static
         *   `-/index.html
         * `-/secret.txt
         */
        assert!(static_dir
            .find_file(&Uri::new(b"/static/../../secret.txt"))
            .is_err());
        Ok(())
    }
}
