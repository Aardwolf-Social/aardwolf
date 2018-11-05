#![allow(proc_macro_derive_resolution_fallback)]
use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};

use super::Post;
use file::File;
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
    pub fn insert(self, conn: &PgConnection) -> Result<MediaPost, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(media_posts::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn new(file: &File, post: &Post) -> Self {
        NewMediaPost {
            file_id: file.id(),
            post_id: post.id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_helper::*;

    #[test]
    fn create_media_post() {
        with_connection(|conn| {
            make_post(conn, |post| {
                with_file(conn, |file| with_media_post(conn, &file, &post, |_| Ok(())))
            })
        })
    }
}
