use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};
use serde_json::Value;

use sql_types::{FollowPolicy, Url};

pub mod follow_request;
pub mod follower;
pub mod group;
pub mod persona;

use self::follower::Follower;
use schema::base_actors;
use user::UserLike;

#[derive(Debug, AsChangeset)]
#[table_name = "base_actors"]
pub struct ModifiedBaseActor {
    id: i32,
    display_name: String,
    profile_url: Url,
    inbox_url: Url,
    outbox_url: Url,
    follow_policy: FollowPolicy,
    original_json: Value,
}

impl ModifiedBaseActor {
    pub fn set_display_name(&mut self, display_name: String) {
        self.display_name = display_name;
    }

    pub fn set_profile_url<U: Into<Url>>(&mut self, profile_url: U) {
        self.profile_url = profile_url.into();
    }

    pub fn set_inbox_url<U: Into<Url>>(&mut self, inbox_url: U) {
        self.inbox_url = inbox_url.into();
    }

    pub fn set_outbox_url<U: Into<Url>>(&mut self, outbox_url: U) {
        self.outbox_url = outbox_url.into();
    }

    pub fn set_follow_policy(&mut self, follow_policy: FollowPolicy) {
        self.follow_policy = follow_policy;
    }

    pub fn save_changes(self, conn: &PgConnection) -> Result<BaseActor, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::update(base_actors::table)
            .set(&self)
            .get_result(conn)
    }
}

#[derive(Debug, Queryable, QueryableByName)]
#[table_name = "base_actors"]
pub struct BaseActor {
    id: i32,
    display_name: String,        // max_length: 80
    profile_url: Url,            // max_length: 2048
    inbox_url: Url,              // max_length: 2048
    outbox_url: Url,             // max_length: 2048
    local_user: Option<i32>,     // foreign key to User
    follow_policy: FollowPolicy, // max_length: 8
    original_json: Value,        // original json
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl BaseActor {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn modify(self) -> ModifiedBaseActor {
        ModifiedBaseActor {
            id: self.id,
            display_name: self.display_name,
            profile_url: self.profile_url,
            inbox_url: self.inbox_url,
            outbox_url: self.outbox_url,
            follow_policy: self.follow_policy,
            original_json: self.original_json,
        }
    }

    pub fn is_following(
        &self,
        follows: &BaseActor,
        conn: &PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        self.is_following_id(follows.id, conn)
    }

    pub fn is_following_id(
        &self,
        follows: i32,
        conn: &PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        use diesel::prelude::*;
        use schema::followers;

        followers::table
            .filter(followers::dsl::follower.eq(self.id))
            .filter(followers::dsl::follows.eq(follows))
            .get_result(conn)
            .map(|_: Follower| true)
            .or_else(|e| match e {
                diesel::result::Error::NotFound => Ok(false),
                e => Err(e),
            })
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn profile_url(&self) -> &Url {
        &self.profile_url
    }

    pub fn inbox_url(&self) -> &Url {
        &self.inbox_url
    }

    pub fn outbox_url(&self) -> &Url {
        &self.outbox_url
    }

    pub fn local_user(&self) -> Option<i32> {
        self.local_user
    }

    pub fn follow_policy(&self) -> FollowPolicy {
        self.follow_policy
    }

    pub fn original_json(&self) -> &Value {
        &self.original_json
    }
}

#[derive(Insertable)]
#[table_name = "base_actors"]
pub struct NewBaseActor {
    display_name: String,
    profile_url: Url,
    inbox_url: Url,
    outbox_url: Url,
    local_user: Option<i32>,
    follow_policy: FollowPolicy,
    original_json: Value,
}

impl NewBaseActor {
    pub fn insert(self, conn: &PgConnection) -> Result<BaseActor, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(base_actors::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn new<U: UserLike>(
        display_name: String,
        profile_url: Url,
        inbox_url: Url,
        outbox_url: Url,
        local_user: Option<&U>,
        follow_policy: FollowPolicy,
        original_json: Value,
    ) -> Self {
        NewBaseActor {
            display_name,
            profile_url: profile_url.into(),
            inbox_url: inbox_url.into(),
            outbox_url: outbox_url.into(),
            local_user: local_user.map(|lu| lu.id()),
            follow_policy,
            original_json,
        }
    }
}

#[cfg(test)]
mod tests {
    use test_helper::*;

    #[test]
    fn create_base_actor() {
        with_connection(|conn| with_base_actor(conn, |_| Ok(())))
    }
}
