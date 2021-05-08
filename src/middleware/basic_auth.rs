use crate::{
    header::HeaderName, middleware::Middleware, request::Request, response::Response,
    status::StatusCode, Uri,
};
use async_trait::async_trait;

use super::MiddlewareChain;

#[derive(Debug)]
pub struct BasicAuth {
    credential_hash: Vec<u8>,
    auth_root: Uri,
}

impl BasicAuth {
    /// Create new Basic authentication middleware. `auth_root` is the root of subtree to protect
    pub fn new(username: &str, password: &str, auth_root: impl Into<Uri>) -> Self {
        let credential = format!("{}:{}", username, password);
        let credential_hash = base64::encode(credential.as_bytes()).into_bytes();
        Self {
            credential_hash,
            auth_root: auth_root.into(),
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
impl<State> Middleware<State> for BasicAuth
where
    State: Clone + Send + Sync + 'static,
{
    async fn call(
        &self,
        request: Request,
        state: State,
        next: MiddlewareChain<'_, State>,
    ) -> Response
    where
        State: Clone + Send + Sync + 'static,
    {
        let uri = request.uri();
        if self.is_protected_uri(uri) {
            if let Err(code) = self.check_credential(&request) {
                assert_eq!(StatusCode::Unauthorized, code);
                let mut response = Response::from(code);
                response.set_header(HeaderName::WwwAuthenticate, "Basic");
                return response;
            }
        }
        next.run(request, state).await
    }
}

#[cfg(test)]
mod tests {
    use crate::{method::Method, server::ServerBuilder};

    use super::*;

    fn server() -> ServerBuilder<()> {
        ServerBuilder::new().route("/", Method::Get, |_, _| async { "Hello" })
    }

    #[tokio::test]
    async fn simple_basic_auth() {
        let server = server().with(BasicAuth::new("user", "pass", "/")).build();
        let request = Request::builder()
            .set_header(HeaderName::Authorization, "Basic dXNlcjpwYXNz")
            .build();
        let response = server.respond(request).await;
        assert_eq!(StatusCode::Ok, response.status_code());
    }

    #[tokio::test]
    async fn not_protected_by_basic_auth() {
        let server = server()
            .with(BasicAuth::new("user", "pass", "/example"))
            .build();
        let request = Request::default();
        let response = server.respond(request).await;
        assert_eq!(StatusCode::Ok, response.status_code());
    }

    #[tokio::test]
    async fn fail_basic_auth() {
        let server = server().with(BasicAuth::new("user", "pass", "/")).build();
        let request = Request::builder()
            .set_header(HeaderName::Authorization, "Basic wrong_hash")
            .build();
        let response = server.respond(request).await;
        assert_eq!(StatusCode::Unauthorized, response.status_code());
        assert_eq!(
            Some(&b"Basic".to_vec()),
            response.get_header(&HeaderName::WwwAuthenticate)
        );
    }
}
