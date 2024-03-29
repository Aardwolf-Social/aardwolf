use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};

use crate::{base_actor::BaseActor, base_post::BasePost, schema::direct_posts};

#[derive(Debug, Queryable, QueryableByName)]
#[diesel(table_name = direct_posts)]
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
        conn: &mut PgConnection,
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

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn base_post_id(&self) -> i32 {
        self.base_post_id
    }

    pub fn base_actor_id(&self) -> i32 {
        self.base_actor_id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = direct_posts)]
pub struct NewDirectPost {
    base_post_id: i32,
    base_actor_id: i32,
}

impl NewDirectPost {
    pub fn insert(self, conn: &mut PgConnection) -> Result<DirectPost, diesel::result::Error> {
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
    use crate::test_helper::*;

    #[test]
    fn create_direct_post() {
        with_connection(|conn| {
            let post_author = make_base_actor(conn)?;
            let base_post = make_base_post(conn, &post_author)?;
            let viewer = make_base_actor(conn)?;
            let post = make_direct_post(conn, &base_post, &viewer);

            assert!(post.is_ok());

            Ok(())
        })
    }
}
