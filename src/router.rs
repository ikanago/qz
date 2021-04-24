use crate::handler::Handler;

/// Associates URI with `Handler`.
/// URI paths are represented as trie tree.
/// This struct is a node of the tree.
#[derive(Debug)]
pub struct Router {
    pub path: Vec<u8>,
    pub handler: Option<Box<dyn Handler>>,
    children: Vec<Box<Router>>,
}

/// Check if the path has wild card at the end of the path.
fn includes_wildcard(path: &[u8]) -> bool {
    path.ends_with(b"/*")
}

impl Router {
    pub fn new() -> Self {
        Self {
            path: Vec::new(),
            handler: None,
            children: Vec::new(),
        }
    }

    fn new_child<F: Handler>(path: &[u8], handler: F) -> Self {
        let mut child = Self {
            path: path.to_vec(),
            handler: Some(Box::new(handler)),
            children: Vec::new(),
        };
        if includes_wildcard(path) && !path.starts_with(b"*") {
            child.split_wildcard();
        }
        child
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

    pub fn add_route<B: AsRef<[u8]>, F: Handler>(&mut self, new_path: B, handler: F) {
        let new_path = new_path.as_ref();
        // For the first time to insert node to root.
        if self.path.len() == 0 && self.children.len() == 0 {
            self.children
                .push(Box::new(Router::new_child(new_path, handler)));
            return;
        }
        if self.path == new_path {
            self.handler = Some(Box::new(handler));
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
                handler: std::mem::take(&mut self.handler),
                children: std::mem::take(&mut self.children),
            };
            if new_path_remaining.len() > 0 {
                // e.g. "abc" and "ade".
                self.children = vec![
                    Box::new(deriving_child),
                    Box::new(Router::new_child(new_path_remaining, handler)),
                ];
            } else {
                // e.g. "abc" and "a".
                // If "a" is inserted in the same way as previous `if` block, a handler for the node "a"
                // is replaced with `None` but the node has a `handler`.
                self.handler = Some(Box::new(handler));
                self.children = vec![Box::new(deriving_child)];
            }
        } else {
            // When longest common prefix of `new_path` is exactly the same as `self.path`.
            // e.g. `self.path`: "static" and `new_path`: "static/index.html"
            let new_path_remaining = &new_path[lcp..];
            for child in &mut self.children {
                match (*child).path.iter().next() {
                    // Because more than 2 children node do not have same prefix,
                    // just check first character of key for each child.
                    Some(first_char) if first_char == new_path_remaining.iter().next().unwrap() => {
                        child.add_route(new_path_remaining, handler);
                        return;
                    }
                    _ => continue,
                }
            }
            // If there is no child in `self.children` that matches new path, just insert it.
            self.children
                .push(Box::new(Router::new_child(new_path_remaining, handler)));
        }
    }

    fn split_wildcard(&mut self) {
        assert!(includes_wildcard(&self.path));
        assert!(self.path.len() > 2);
        let (_, path) = self.path.split_last().unwrap();
        self.path = path.to_vec();
        self.children.push(Box::new(Self {
            path: b"*".to_vec(),
            handler: None,
            children: Vec::new(),
        }));
    }

    pub fn find<B: AsRef<[u8]>>(&self, key: B) -> Option<&Box<dyn Handler>> {
        let key = key.as_ref();
        if key.len() == 0 {
            return None;
        }
        if &self.path[..] > key {
            // e.g. `self.path` is "hoge" and `key` is "ho".
            return None;
        }
        if self.path == key {
            return self.handler.as_ref();
        }

        let lcp = self.longest_common_prefix(key);
        let key_remaining = &key[lcp..];

        for child in &self.children {
            if &child.path == b"*" {
                return self.handler.as_ref();
            }
            match (*child).path.iter().next() {
                // Because more than 2 children node do not have the same prefix,
                // just check first character of key for each child.
                Some(first_char) if first_char == key_remaining.iter().next().unwrap() => {
                    return child.find(key_remaining);
                }
                _ => continue,
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{request::Request, response::Responder};

    #[test]
    fn lcp() {
        let node_x = Router {
            path: b"abcde".to_vec(),
            handler: None,
            children: Vec::new(),
        };
        assert_eq!(node_x.longest_common_prefix(b"abchoge"), 3);
    }

    #[test]
    fn lcp_root() {
        let node_x = Router {
            path: b"".to_vec(),
            handler: None,
            children: Vec::new(),
        };
        assert_eq!(node_x.longest_common_prefix(b"abchoge"), 0);
    }

    async fn dummy_handler(_request: Request) -> impl Responder {
        unimplemented!()
    }

    #[test]
    fn find() {
        let mut tree = Router::new();
        let keys = vec!["/", "to", "tea", "ted", "hoge", "h", "i", "in", "inn"];
        for key in &keys {
            tree.add_route(key.as_bytes(), dummy_handler);
        }
        for key in keys {
            tree.find(key.as_bytes()).unwrap();
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
            tree.add_route(key, dummy_handler);
        }
        for key in &keys {
            tree.find(key).unwrap();
        }
    }

    #[test]
    fn find_with_wildcard() {
        let mut tree = Router::new();
        let paths = vec!["/", "/index.html", "/static/*"];
        for key in &paths {
            tree.add_route(key.as_bytes(), dummy_handler);
        }
        let queries = vec![
            "/",
            "/index.html",
            "/static/index.html",
            "/static/style.css",
            "/static/index.js",
        ];
        for query in &queries {
            tree.find(query.as_bytes()).unwrap();
        }
    }

    #[test]
    fn dont_match_substr() {
        let mut tree = Router::new();
        tree.add_route(b"/hoge", dummy_handler);
        assert!(tree.find(b"/ho").is_none())
    }
}
