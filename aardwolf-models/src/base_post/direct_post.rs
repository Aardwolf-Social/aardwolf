use chrono::DateTime;
use chrono::offset::Utc;
use diesel;
use diesel::pg::PgConnection;

use base_actor::BaseActor;
use base_post::BasePost;
use schema::direct_posts;

#[derive(Debug, Queryable, QueryableByName)]
#[table_name = "direct_posts"]
pub struct DirectPost {
    id: i32,
    base_post_id: i32,
    base_actor_id: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl DirectPost {
    pub fn exists(
        actor: &BaseActor,
        post: &BasePost,
        conn: &PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        use diesel::prelude::*;

        direct_posts::table
            .filter(direct_posts::dsl::base_actor_id.eq(actor.id()))
            .filter(direct_posts::dsl::base_post_id.eq(post.id()))
            .get_result(conn)
            .map(|_: DirectPost| true)
            .or_else(|e| match e {
                diesel::result::Error::NotFound => Ok(false),
                e => Err(e),
            })
    }
}
