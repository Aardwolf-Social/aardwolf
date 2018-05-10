use chrono::DateTime;
use chrono::offset::Utc;
use diesel;
use diesel::pg::PgConnection;
use mime::Mime as OrigMime;
use serde_json::Value;

pub mod direct_post;
pub mod post;

use base_actor::BaseActor;
use file::image::Image;
use schema::base_posts;
use self::direct_post::DirectPost;
use sql_types::{Mime, PostVisibility};

#[derive(Debug, Queryable, QueryableByName)]
#[table_name = "base_posts"]
pub struct BasePost {
    id: i32,
    name: Option<String>, // max_length: 140
    media_type: Mime,     // max_length: 80
    posted_by: i32,       // foreign key to BaseActor
    icon: Option<i32>,    // foreign key to Image
    visibility: PostVisibility,
    original_json: Value, // original json
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl BasePost {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_ref())
    }

    pub fn media_type(&self) -> &Mime {
        &self.media_type
    }

    pub fn posted_by(&self) -> i32 {
        self.posted_by
    }

    pub fn icon(&self) -> Option<i32> {
        self.icon
    }

    pub fn visibility(&self) -> PostVisibility {
        self.visibility
    }

    pub fn original_json(&self) -> &Value {
        &self.original_json
    }

    pub fn is_visible_by(
        &self,
        actor: &BaseActor,
        conn: &PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        match self.visibility {
            PostVisibility::Public => Ok(true),
            PostVisibility::FollowersOnly => actor.is_following_id(self.posted_by, conn),
            PostVisibility::ListedPeopleOnly => DirectPost::exists(actor, self, conn),
            _ => Ok(false),
        }
    }
}

#[derive(Insertable)]
#[table_name = "base_posts"]
pub struct NewBasePost {
    name: Option<String>,
    media_type: Mime,
    posted_by: i32,
    icon: Option<i32>,
    visibility: PostVisibility,
    original_json: Value,
}

impl NewBasePost {
    pub fn insert(self, conn: &PgConnection) -> Result<BasePost, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(base_posts::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn new(
        name: Option<String>,
        media_type: OrigMime,
        posted_by: &BaseActor,
        icon: Option<&Image>,
        visibility: PostVisibility,
        original_json: Value,
    ) -> Self {
        NewBasePost {
            name,
            media_type: media_type.into(),
            posted_by: posted_by.id(),
            icon: icon.map(|i| i.id()),
            visibility,
            original_json,
        }
    }
}

#[cfg(test)]
mod tests {
    use test_helper::*;

    #[test]
    fn create_base_post() {
        with_connection(|conn| {
            with_base_actor(conn, |posted_by| {
                with_base_post(conn, &posted_by, |_| Ok(()))
            })
        })
    }
}
