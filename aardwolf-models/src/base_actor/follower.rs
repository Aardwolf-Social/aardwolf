use chrono::DateTime;
use chrono::offset::Utc;
use diesel;
use diesel::pg::PgConnection;

use base_actor::BaseActor;
use base_actor::follow_request::FollowRequest;
use schema::followers;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "followers"]
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
}

#[derive(Insertable)]
#[table_name = "followers"]
pub struct NewFollower {
    follower: i32,
    follows: i32,
}

impl NewFollower {
    pub fn insert(self, conn: &PgConnection) -> Result<Follower, diesel::result::Error> {
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
    use test_helper::*;

    #[test]
    fn create_follower() {
        with_connection(|conn| {
            with_base_actor(conn, |follower| {
                with_base_actor(conn, |follows| {
                    with_follower(conn, &follower, &follows, |_| Ok(()))
                })
            })
        })
    }
}
