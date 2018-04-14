use chrono::DateTime;
use chrono::offset::Utc;

use file::File;
use super::Post;
use schema::media_posts;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "media_posts"]
pub struct MediaPost {
    id: i32,
    file_id: i32, // foreign key to File
    post_id: i32, // foreign key to Post
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl MediaPost {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn file_id(&self) -> i32 {
        self.file_id
    }

    pub fn post_id(&self) -> i32 {
        self.post_id
    }
}

#[derive(Insertable)]
#[table_name = "media_posts"]
pub struct NewMediaPost {
    file_id: i32,
    post_id: i32,
}

impl NewMediaPost {
    pub fn new(file: &File, post: &Post) -> Self {
        NewMediaPost {
            file_id: file.id(),
            post_id: post.id(),
        }
    }
}
