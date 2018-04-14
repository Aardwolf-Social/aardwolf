use chrono::DateTime;
use chrono::offset::Utc;

use super::Post;
use schema::comments;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "comments"]
pub struct Comment {
    id: i32,
    conversation: i32, // foreign key to topic Post
    parent: i32,       // foreign key to replied Post
    post: i32,         // foreign key to Post
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Comment {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn conversation(&self) -> i32 {
        self.conversation
    }

    pub fn parent(&self) -> i32 {
        self.parent
    }

    pub fn post(&self) -> i32 {
        self.post
    }
}

#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    conversation: i32,
    parent: i32,
    post: i32,
}

impl NewComment {
    pub fn new(conversation: &Post, parent: &Post, post: &Post) -> Self {
        NewComment {
            conversation: conversation.id(),
            parent: parent.id(),
            post: post.id(),
        }
    }
}
