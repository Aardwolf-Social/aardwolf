use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};

use super::Comment;
use schema::reactions;
use sql_types::ReactionType;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "reactions"]
pub struct Reaction {
    id: i32,
    reaction_type: ReactionType,
    comment_id: i32, // foreign key to Comment
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Reaction {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn reaction_type(&self) -> ReactionType {
        self.reaction_type
    }

    pub fn comment_id(&self) -> i32 {
        self.comment_id
    }
}

#[derive(Insertable)]
#[table_name = "reactions"]
pub struct NewReaction {
    reaction_type: ReactionType,
    comment_id: i32,
}

impl NewReaction {
    pub fn insert(self, conn: &PgConnection) -> Result<Reaction, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(reactions::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn new(reaction_type: ReactionType, comment: &Comment) -> Self {
        NewReaction {
            reaction_type,
            comment_id: comment.id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_helper::*;

    #[test]
    fn create_reaction() {
        with_connection(|conn| {
            make_post(conn, |conversation_post| {
                make_post(conn, |comment_post| {
                    with_comment(
                        conn,
                        &conversation_post,
                        &conversation_post,
                        &comment_post,
                        |comment| with_reaction(conn, &comment, |_| Ok(())),
                    )
                })
            })
        })
    }
}
