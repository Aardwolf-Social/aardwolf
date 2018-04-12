use chrono::DateTime;
use chrono::offset::Utc;

use base_actor::BaseActor;
use schema::follow_requests;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "follow_requests"]
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
}

#[derive(Insertable)]
#[table_name = "follow_requests"]
pub struct NewFollowRequest {
    follower: i32,
    requested_follow: i32,
}

impl NewFollowRequest {
    pub fn new(follower: &BaseActor, requested_follow: &BaseActor) -> Self {
        NewFollowRequest {
            follower: follower.id(),
            requested_follow: requested_follow.id(),
        }
    }
}
