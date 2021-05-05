use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    username: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    username: String,
    text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Posts {
    posts: Vec<Post>,
}

impl Posts {
    pub fn new(posts: Vec<Post>) -> Self {
        Self { posts }
    }
}
