use crate::{handler::Handler, method::Method, status::StatusCode};
use std::collections::HashMap;

/// Associates URI with `Handler`.
/// URI paths are represented as trie tree.
/// This struct is a node of the tree.
#[derive(Debug)]
pub struct Router<State>
where
    State: Clone + Send + Sync + 'static,
{
    path: Vec<u8>,
    handlers: HashMap<Method, Box<dyn Handler<State>>>,
    children: Vec<Router<State>>,
}

/// Check if the path has wild card at the end of the path.
fn includes_wildcard(path: &[u8]) -> bool {
    path.ends_with(b"/*")
}

impl<State> Router<State>
where
    State: Clone + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            path: Vec::new(),
            handlers: HashMap::new(),
            children: Vec::new(),
        }
    }

    fn new_child<F: Handler<State>>(path: &[u8], method: Method, handler: F) -> Self {
        if includes_wildcard(path) && !path.starts_with(b"*") {
            let mut child = Self {
                path: path.to_vec(),
                handlers: HashMap::new(),
                children: Vec::new(),
            };
            child.split_wildcard(method, handler);
            child
        } else {
            let mut handlers: HashMap<Method, Box<dyn Handler<State> + 'static>> = HashMap::new();
            handlers.insert(method, Box::new(handler));
            Self {
                path: path.to_vec(),
                handlers,
                children: Vec::new(),
            }
        }
    }

    fn split_wildcard<F: Handler<State>>(&mut self, method: Method, handler: F) {
        assert!(includes_wildcard(&self.path));
        assert!(self.path.len() >= 2);
        let (_, path) = self.path.split_last().unwrap();
        self.path = path.to_vec();
        let mut handlers: HashMap<Method, Box<dyn Handler<State> + 'static>> = HashMap::new();
        handlers.insert(method, Box::new(handler));
        self.children.push(Self {
            path: b"*".to_vec(),
            handlers,
            children: Vec::new(),
        });
    }

    /// Return how many common character path of `Route` nodes and an arugument have.
    fn longest_common_prefix(&self, other: &[u8]) -> usize {
        let mut pos = 0;
        for (char_self, char_other) in self.path.iter().zip(other.iter()) {
            if char_self == char_other {
                pos += 1;
            } else {
                break;
            }
        }
        pos
    }

    pub fn add_route<B: AsRef<[u8]>, F: Handler<State>>(
        &mut self,
        new_path: B,
        method: Method,
        handler: F,
    ) {
        let new_path = new_path.as_ref();
        // For the first time to insert node to root.
        if self.path.is_empty() && self.children.is_empty() {
            self.children
                .push(Router::new_child(new_path, method, handler));
            return;
        }
        if self.path == new_path {
            self.handlers.insert(method, Box::new(handler));
            return;
        }

        let lcp = self.longest_common_prefix(new_path);
        if self.path.len() > lcp {
            // e.g. `self.path` is "static" and longest common prefix is "stat".
            let path = self.path.clone();
            let common_prefix = &path[..lcp];
            let path_remaining = &path[lcp..];
            let new_path_remaining = &new_path[lcp..];

            self.path = common_prefix.to_vec();
            let deriving_child = Self {
                path: path_remaining.to_vec(),
                handlers: std::mem::take(&mut self.handlers),
                children: std::mem::take(&mut self.children),
            };
            if !new_path_remaining.is_empty() {
                // e.g. "abc" and "ade".
                self.children = vec![
                    deriving_child,
                    Router::new_child(new_path_remaining, method, handler),
                ];
            } else {
                // e.g. "abc" and "a".
                // If "a" is inserted in the same way as previous `if` block, a handler for the node "a"
                // is replaced with `None` but the node has a `handler`.
                self.handlers.insert(method, Box::new(handler));
                self.children = vec![deriving_child];
            }
        } else {
            // When longest common prefix of `new_path` is exactly the same as `self.path`.
            // e.g. `self.path`: "static" and `new_path`: "static/index.html"
            let new_path_remaining = &new_path[lcp..];
            for child in &mut self.children {
                match (*child).path.get(0) {
                    // Because more than 2 children node do not have same prefix,
                    // just check first character of key for each child.
                    Some(first_char) if first_char == new_path_remaining.iter().next().unwrap() => {
                        child.add_route(new_path_remaining, method, handler);
                        return;
                    }
                    _ => continue,
                }
            }
            // If there is no child in `self.children` that matches new path, just insert it.
            self.children
                .push(Router::new_child(new_path_remaining, method, handler));
        }
    }

    pub fn find<B: AsRef<[u8]>>(
        &self,
        key: B,
        method: Method,
    ) -> crate::Result<&Box<dyn Handler<State>>> {
        let key = key.as_ref();
        if key.is_empty() {
            return Err(StatusCode::NotFound);
        }
        if &self.path[..] > key {
            // e.g. `self.path` is "hoge" and `key` is "ho".
            return Err(StatusCode::NotFound);
        }
        if key == self.path {
            match self.handlers.get(&method) {
                Some(handler) => return Ok(&handler),
                None => {
                    if self.children.is_empty() {
                        return Err(StatusCode::MethodNotAllowed);
                    }
                    // Try further e.g. `self.path` is "hoge", key is "hoge" and this node has
                    // wildcard child.
                }
            }
        }

        let lcp = self.longest_common_prefix(key);
        let key_remaining = &key[lcp..];
        for child in &self.children {
            if &child.path == b"*" {
                match child.handlers.get(&method) {
                    Some(handler) => return Ok(&handler),
                    None => return Err(StatusCode::MethodNotAllowed),
                }
            }
            if let (Some(c), Some(d)) = (child.path.get(0), key_remaining.iter().next()) {
                if c == d {
                    return child.find(key_remaining, method);
                }
            }
        }
        Err(StatusCode::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{body::Body, request::Request, response::Response};
    use async_trait::async_trait;

    #[test]
    fn lcp() {
        let node_x = Router::<()> {
            path: b"abcde".to_vec(),
            handlers: HashMap::new(),
            children: Vec::new(),
        };
        assert_eq!(node_x.longest_common_prefix(b"abchoge"), 3);
    }

    #[test]
    fn lcp_root() {
        let node_x = Router::<()> {
            path: b"".to_vec(),
            handlers: HashMap::new(),
            children: Vec::new(),
        };
        assert_eq!(node_x.longest_common_prefix(b"abchoge"), 0);
    }

    macro_rules! impl_dummy_handler {
        ($id: ident, $body:expr) => {
            #[derive(Clone, Debug, PartialEq, Eq)]
            struct $id;

            #[async_trait]
            impl Handler<()> for $id {
                async fn call(&self, _request: Request, _state: ()) -> crate::Result<Response> {
                    let mut response = Response::default();
                    response.set_body($body.as_bytes().to_vec());
                    Ok(response)
                }
            }
        };
    }

    impl_dummy_handler!(Dummy, "dummy");

    #[test]
    fn find() {
        let mut tree = Router::new();
        let keys = vec!["/", "to", "tea", "ted", "hoge", "h", "i", "in", "inn"];
        for key in &keys {
            tree.add_route(key.as_bytes(), Method::Get, Dummy);
        }
        for key in keys {
            tree.find(key.as_bytes(), Method::Get).unwrap();
        }
    }

    #[test]
    fn invalid_method() {
        let mut tree = Router::new();
        tree.add_route("/example", Method::Get, Dummy);
        match tree.find(b"/example", Method::Post) {
            Ok(_) => panic!("No handler is registered for POST"),
            Err(code) => assert_eq!(StatusCode::MethodNotAllowed, code),
        }
    }

    // Generate random alphanumeric string.
    fn random_bytes() -> Vec<u8> {
        use rand::distributions::Alphanumeric;
        use rand::random;
        use rand::Rng;
        let length = random::<usize>() % 100 + 1;
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .collect::<Vec<_>>()
    }

    #[test]
    fn find_random() {
        let mut tree = Router::new();
        let count = 1000;
        let keys = (0..count).map(|_| random_bytes()).collect::<Vec<Vec<_>>>();
        for key in &keys {
            tree.add_route(key, Method::Get, Dummy);
        }
        for key in &keys {
            tree.find(key, Method::Get).unwrap();
        }
    }

    #[test]
    fn find_with_wildcard() {
        let mut tree = Router::new();
        let paths = vec!["/", "/index.html", "/static/*"];
        for key in &paths {
            tree.add_route(key.as_bytes(), Method::Get, Dummy);
        }
        let queries = vec![
            "/",
            "/index.html",
            "/static/index.html",
            "/static/style.css",
            "/static/index.js",
        ];
        for query in &queries {
            tree.find(query.as_bytes(), Method::Get).unwrap();
        }
    }

    #[test]
    fn dont_match_substr() {
        let mut tree = Router::new();
        tree.add_route(b"/hoge", Method::Get, Dummy);
        assert!(tree.find(b"/ho", Method::Get).is_err())
    }

    #[test]
    fn dont_match_substr2() {
        let mut tree = Router::new();
        tree.add_route(b"/hoge", Method::Get, Dummy);
        tree.add_route(b"/ho", Method::Post, Dummy);
        assert!(tree.find(b"/ho", Method::Get).is_err())
    }

    #[test]
    fn match_child_wildcard_defferent_method() {
        let mut tree = Router::new();
        tree.add_route(b"/hoge", Method::Get, Dummy);
        tree.add_route(b"/hoge*", Method::Post, Dummy);
        assert!(tree.find(b"/hoge", Method::Post).is_ok())
    }

    async fn extract_body(tree: &Router<()>, key: &[u8], method: Method) -> Body {
        tree.find(key, method)
            .unwrap()
            .call(Request::default(), ())
            .await
            .unwrap()
            .body()
            .clone()
    }

    #[tokio::test]
    async fn longest_match_wildcard() {
        impl_dummy_handler!(Root, "root");
        impl_dummy_handler!(Wildcard, "wildcard");

        let mut tree = Router::new();
        tree.add_route(b"/", Method::Get, Root);
        tree.add_route(b"/*", Method::Get, Wildcard);
        // Handlers are dispatched dynamically in `Router`, so I have no idea to compare
        // them. They are distinguished with response body this functions returns.
        assert_eq!(
            Body::Some(b"root".to_vec()),
            extract_body(&tree, b"/", Method::Get).await
        );
        assert_eq!(
            Body::Some(b"wildcard".to_vec()),
            extract_body(&tree, b"/hoge", Method::Get).await
        );
    }

    #[tokio::test]
    async fn different_method_to_a_uri() {
        impl_dummy_handler!(Get, "get");
        impl_dummy_handler!(Post, "post");

        let mut tree = Router::new();
        tree.add_route(b"/", Method::Get, Get);
        tree.add_route(b"/", Method::Post, Post);
        // Handlers are dispatched dynamically in `Router`, so I have no idea to compare
        // them. They are distinguished with response body this functions returns.
        assert_eq!(
            Body::Some(b"get".to_vec()),
            extract_body(&tree, b"/", Method::Get).await
        );
        assert_eq!(
            Body::Some(b"post".to_vec()),
            extract_body(&tree, b"/", Method::Post).await
        );
    }
}
