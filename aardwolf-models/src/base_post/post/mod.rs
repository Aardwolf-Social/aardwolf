use chrono::DateTime;
use chrono::offset::Utc;

use base_post::BasePost;
use schema::posts;

pub mod comment;
pub mod media_post;

#[derive(Debug, Queryable, QueryableByName)]
#[table_name = "posts"]
pub struct Post {
    id: i32,
    content: String,
    source: Option<String>,
    base_post: i32, // foreign key to BasePost
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Post {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn source(&self) -> Option<&str> {
        self.source.as_ref().map(|s| s.as_ref())
    }

    pub fn base_post(&self) -> i32 {
        self.base_post
    }
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    content: String,
    source: Option<String>,
    base_post: i32,
}

impl NewPost {
    pub fn new(content: String, source: Option<String>, base_post: &BasePost) -> Self {
        NewPost {
            content,
            source,
            base_post: base_post.id(),
        }
    }
}
