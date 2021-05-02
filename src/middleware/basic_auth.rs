use crate::{
    header::HeaderName, middleware::Middleware, request::Request, response::Response,
    status::StatusCode, Uri,
};
use async_trait::async_trait;
use base64;

#[derive(Debug)]
pub struct BasicAuth {
    credential_hash: Vec<u8>,
    auth_root: Uri,
}

impl BasicAuth {
    /// Create new Basic authentication middleware. `auth_root` is the root of subtree to protect
    pub fn new(username: &str, password: &str, auth_root: Uri) -> Self {
        let credential = format!("{}:{}", username, password);
        let credential_hash = base64::encode(credential.as_bytes()).into_bytes();
        Self {
            credential_hash,
            auth_root,
        }
    }

    fn is_protected_uri(&self, uri: &Uri) -> bool {
        uri.0.starts_with(&self.auth_root.0)
    }

    fn check_credential(&self, request: &Request) -> crate::Result<()> {
        let credential = request
            .get_header(HeaderName::Authorization)
            .ok_or(StatusCode::Unauthorized)?;
        // credential = [b"Basic:", b"xxxxxxx"]
        let mut credential = credential.split(|&c| c == b' ');
        credential
            .next()
            .and_then(|c| if c == b"Basic" { Some(()) } else { None })
            .ok_or(StatusCode::Unauthorized)?;
        credential
            .next()
            .and_then(|hash| {
                if hash == self.credential_hash {
                    Some(())
                } else {
                    None
                }
            })
            .ok_or(StatusCode::Unauthorized)?;
        Ok(())
    }
}

#[async_trait]
impl Middleware for BasicAuth {
    async fn call(&self, request: Request) -> crate::Result<Request, Response> {
        let uri = request.uri();
        if self.is_protected_uri(uri) {
            if let Err(code) = self.check_credential(&request) {
                assert_eq!(StatusCode::Unauthorized, code);
                let mut response = Response::from(code);
                response.set_header(HeaderName::WwwAuthenticate, "Basic");
                return Err(response);
            }
        }
        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn simple_basic_auth() {
        let basic_auth = BasicAuth::new("user", "pass", Uri::from("/"));
        let request = Request::builder()
            .set_header(HeaderName::Authorization, "Basic dXNlcjpwYXNz")
            .build();
        basic_auth.call(request).await.unwrap();
    }

    #[tokio::test]
    async fn not_protected_by_basic_auth() {
        let basic_auth = BasicAuth::new("user", "pass", Uri::from("/example"));
        let request = Request::default();
        basic_auth.call(request).await.unwrap();
    }

    #[tokio::test]
    async fn fail_basic_auth() {
        let basic_auth = BasicAuth::new("user", "pass", Uri::from("/"));
        let request = Request::builder()
            .set_header(HeaderName::Authorization, "Basic wrong_hash")
            .build();
        let response = match basic_auth.call(request).await {
            Ok(_) => unreachable!("Provided hash must be wrong"),
            Err(response) => response,
        };
        assert_eq!(
            Some(&b"Basic".to_vec()),
            response.get_header(&HeaderName::WwwAuthenticate)
        );
    }
}
