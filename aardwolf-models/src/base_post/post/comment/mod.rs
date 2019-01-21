use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};

use crate::{base_post::post::Post, schema::comments};

pub mod reaction;

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
    pub fn insert(self, conn: &PgConnection) -> Result<Comment, diesel::result::Error> {
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
            make_post(conn, |conversation_post| {
                make_post(conn, |comment_post| {
                    with_comment(
                        conn,
                        &conversation_post,
                        &conversation_post,
                        &comment_post,
                        |_| Ok(()),
                    )
                })
            })
        })
    }

    #[test]
    fn create_comment_in_thread() {
        with_connection(|conn| {
            make_post(conn, |conversation_post| {
                make_post(conn, |parent_post| {
                    with_comment(
                        conn,
                        &conversation_post,
                        &conversation_post,
                        &parent_post,
                        |_parent_comment| {
                            make_post(conn, |comment_post| {
                                with_comment(
                                    conn,
                                    &conversation_post,
                                    &parent_post,
                                    &comment_post,
                                    |_| Ok(()),
                                )
                            })
                        },
                    )
                })
            })
        })
    }
}
