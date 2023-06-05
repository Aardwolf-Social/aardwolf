use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};

use crate::{base_actor::BaseActor, schema::follow_requests};

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[diesel(table_name = follow_requests)]
pub struct FollowRequest {
    id: i32,
    follower: i32,         // foreign key to BaseActor
    requested_follow: i32, // foreign key to BaseActor
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl FollowRequest {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn follower(&self) -> i32 {
        self.follower
    }

    pub fn requested_follow(&self) -> i32 {
        self.requested_follow
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Insertable)]
#[diesel(table_name = follow_requests)]
pub struct NewFollowRequest {
    follower: i32,
    requested_follow: i32,
}

impl NewFollowRequest {
    pub fn insert(self, conn: &mut PgConnection) -> Result<FollowRequest, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(follow_requests::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn new(follower: &BaseActor, requested_follow: &BaseActor) -> Self {
        NewFollowRequest {
            follower: follower.id(),
            requested_follow: requested_follow.id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::*;

    #[test]
    fn create_follow_request() {
        with_connection(|conn| {
            let follower_actor = make_base_actor(conn)?;
            let target_actor = make_base_actor(conn)?;

            let request = make_follow_request(conn, &follower_actor, &target_actor);

            assert!(request.is_ok());

            Ok(())
        })
    }
}
