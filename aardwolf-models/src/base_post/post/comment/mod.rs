use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};

use crate::{base_post::post::Post, schema::comments};

pub mod reaction;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[diesel(table_name = comments)]
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

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment {
    conversation: i32,
    parent: i32,
    post: i32,
}

impl NewComment {
    pub fn insert(self, conn: &mut PgConnection) -> Result<Comment, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(comments::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn new(conversation: &Post, parent: &Post, post: &Post) -> Self {
        NewComment {
            conversation: conversation.id(),
            parent: parent.id(),
            post: post.id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::*;

    #[test]
    fn create_comment_on_conversation() {
        with_connection(|conn| {
            let posted_by = make_base_actor(conn)?;
            let base_post = make_base_post(conn, &posted_by)?;

            let conversation_post = make_post_with(conn, &base_post)?;
            let comment_post = make_post_with(conn, &base_post)?;

            let comment = make_comment(conn, &conversation_post, &conversation_post, &comment_post);

            assert!(comment.is_ok());

            Ok(())
        })
    }

    #[test]
    fn create_comment_in_thread() {
        with_connection(|conn| {
            let conversation_post = make_post(conn)?;
            let parent_post = make_post(conn)?;
            let _parent_comment =
                make_comment(conn, &conversation_post, &conversation_post, &parent_post)?;
            let comment_post = make_post(conn)?;

            let comment = make_comment(conn, &conversation_post, &parent_post, &comment_post);

            assert!(comment.is_ok());

            Ok(())
        })
    }
}
