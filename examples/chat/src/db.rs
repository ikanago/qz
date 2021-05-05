use crate::model::{Post, Posts, User};

#[derive(Debug)]
pub struct Db {
    users: Vec<User>,
    posts: Vec<Post>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            posts: Vec::new(),
        }
    }

    pub fn register(&mut self, user: User) {
        self.users.push(user);
    }

    pub fn current_posts(&self, count: usize) -> Posts {
        Posts::new(self.posts.iter().take(count).cloned().collect())
    }

    pub fn create_post(&mut self, post: Post) {
        self.posts.push(post);
    }
}
