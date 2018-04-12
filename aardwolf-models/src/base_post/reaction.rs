use chrono::DateTime;
use chrono::offset::Utc;

use base_post::post::comment::Comment;
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
    pub fn new(reaction_type: ReactionType, comment: &Comment) -> Self {
        NewReaction {
            reaction_type,
            comment_id: comment.id(),
        }
    }
}
