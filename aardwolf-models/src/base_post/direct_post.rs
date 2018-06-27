use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};

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

#[derive(Debug, Insertable)]
#[table_name = "direct_posts"]
pub struct NewDirectPost {
    base_post_id: i32,
    base_actor_id: i32,
}

impl NewDirectPost {
    pub fn insert(self, conn: &PgConnection) -> Result<DirectPost, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(direct_posts::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn new(base_post: &BasePost, base_actor: &BaseActor) -> Self {
        NewDirectPost {
            base_post_id: base_post.id(),
            base_actor_id: base_actor.id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use test_helper::*;

    #[test]
    fn create_direct_post() {
        with_connection(|conn| {
            with_base_actor(conn, |post_author| {
                with_base_post(conn, &post_author, |base_post| {
                    with_base_actor(conn, |viewer| {
                        with_direct_post(conn, &base_post, &viewer, |_| Ok(()))
                    })
                })
            })
        })
    }
}
