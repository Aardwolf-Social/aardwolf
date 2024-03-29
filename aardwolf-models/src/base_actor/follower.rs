use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};

use crate::{
    base_actor::{follow_request::FollowRequest, BaseActor},
    schema::followers,
};

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[diesel(table_name = followers)]
pub struct Follower {
    id: i32,
    follower: i32, // foreign key to BaseActor
    follows: i32,  // foreign key to BaseActor
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Follower {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn follower(&self) -> i32 {
        self.follower
    }

    pub fn follows(&self) -> i32 {
        self.follows
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Insertable)]
#[diesel(table_name = followers)]
pub struct NewFollower {
    follower: i32,
    follows: i32,
}

impl NewFollower {
    pub fn insert(self, conn: &mut PgConnection) -> Result<Follower, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(followers::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn new(follower: &BaseActor, follows: &BaseActor) -> Self {
        NewFollower {
            follower: follower.id(),
            follows: follows.id(),
        }
    }
}

impl From<FollowRequest> for NewFollower {
    fn from(follow_request: FollowRequest) -> Self {
        NewFollower {
            follower: follow_request.follower(),
            follows: follow_request.requested_follow(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::*;

    #[test]
    fn create_follower() {
        with_connection(|conn| {
            let follower_actor = make_base_actor(conn)?;
            let follows = make_base_actor(conn)?;

            let follower = make_follower(conn, &follower_actor, &follows);

            assert!(follower.is_ok());

            Ok(())
        })
    }
}
