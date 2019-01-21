#![allow(proc_macro_derive_resolution_fallback)]
use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};
use mime::Mime as OrigMime;
use uuid::Uuid;

pub mod direct_post;
pub mod post;

use crate::{
    base_actor::BaseActor,
    base_post::direct_post::DirectPost,
    file::image::Image,
    schema::base_posts,
    sql_types::{Mime, PostVisibility},
};

#[derive(Debug, Queryable, QueryableByName)]
#[table_name = "base_posts"]
pub struct BasePost {
    id: i32,
    name: Option<String>, // max_length: 140
    media_type: Mime,     // max_length: 80
    posted_by: i32,       // foreign key to BaseActor
    icon: Option<i32>,    // foreign key to Image
    visibility: PostVisibility,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    local_uuid: Option<Uuid>,
    activitypub_id: String,
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
    local_uuid: Option<Uuid>,
    activitypub_id: String,
}

impl NewBasePost {
    pub fn insert(self, conn: &PgConnection) -> Result<BasePost, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(base_posts::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn local(
        name: Option<String>,
        media_type: OrigMime,
        posted_by: &BaseActor,
        icon: Option<&Image>,
        visibility: PostVisibility,
        generate_id: impl Fn(&Uuid) -> String,
    ) -> Self {
        let uuid = Uuid::new_v4();

        NewBasePost {
            name,
            media_type: media_type.into(),
            posted_by: posted_by.id(),
            icon: icon.map(|i| i.id()),
            visibility,
            activitypub_id: generate_id(&uuid),
            local_uuid: Some(uuid),
        }
    }

    pub fn new(
        name: Option<String>,
        media_type: OrigMime,
        posted_by: &BaseActor,
        icon: Option<&Image>,
        visibility: PostVisibility,
        activitypub_id: String,
    ) -> Self {
        NewBasePost {
            name,
            media_type: media_type.into(),
            posted_by: posted_by.id(),
            icon: icon.map(|i| i.id()),
            visibility,
            local_uuid: None,
            activitypub_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::*;

    #[test]
    fn create_base_post() {
        with_connection(|conn| {
            with_base_actor(conn, |posted_by| {
                with_base_post(conn, &posted_by, |_| Ok(()))
            })
        })
    }
}
