mod static_dir;
mod static_file;

use crate::Uri;
pub use static_dir::StaticDir;
pub use static_file::StaticFile;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

fn find_file(path: &Uri, mount_dir: &Path, serve_at: &Path) -> Result<PathBuf, ()> {
    let path = std::str::from_utf8(&path.0).unwrap();
    let path = Path::new(path);
    let path = match path.strip_prefix(serve_at) {
        Ok(path) => path,
        Err(_) => return Err(()),
    };

    let mut file_to_find = mount_dir.to_path_buf();
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

    if !file_to_find.starts_with(&mount_dir) {
        return Err(());
    }
    if !file_to_find.exists() {
        return Err(());
    }
    Ok(file_to_find)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Uri;
    use tokio::{
        fs::{self, File},
        io,
    };

    // Creates ./static_dir_test/static/index.html for tests.
    // First this function uses `tempfile` crate, but temporary directory which is created with the
    // crate is deleted when it goes out of scope.
    async fn setup_dir() -> io::Result<(PathBuf, PathBuf)> {
        let static_dir = PathBuf::from("./static_dir_test/static");
        fs::create_dir_all(static_dir.clone()).await?;

        let file_path = static_dir.join("index.html");
        File::create(file_path).await?;
        Ok((static_dir, PathBuf::from("/static")))
    }

    #[tokio::test]
    async fn find_static_file() -> io::Result<()> {
        let (static_dir, serve_at) = setup_dir().await?;
        find_file(
            &Uri::new(b"/static/index.html"),
            static_dir.as_path(),
            serve_at.as_path(),
        )
        .unwrap();
        Ok(())
    }

    #[tokio::test]
    async fn find_missing_file() -> io::Result<()> {
        let (static_dir, serve_at) = setup_dir().await?;
        assert!(find_file(
            &Uri::new(b"/static/index.css"),
            static_dir.as_path(),
            serve_at.as_path()
        )
        .is_err());
        Ok(())
    }

    #[tokio::test]
    async fn invalid_access() -> io::Result<()> {
        let (static_dir, serve_at) = setup_dir().await?;
        File::create("./static_dir_test/secret.txt").await?;
        /*
         * /static_dir_test
         * `-/static
         *   `-/index.html
         * `-/secret.txt
         */
        assert!(find_file(
            &Uri::new(b"/static/../../secret.txt"),
            static_dir.as_path(),
            serve_at.as_path()
        )
        .is_err());
        Ok(())
    }
}
