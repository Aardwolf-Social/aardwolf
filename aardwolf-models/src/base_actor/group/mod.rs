use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};

pub mod group_base_actor;

use crate::{base_actor::BaseActor, schema::groups};

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[diesel(table_name = groups)]
pub struct Group {
    id: i32,
    base_actor_id: i32, // foreign key to BaseActor
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Group {
    pub fn id(&self) -> i32 {
        self.id
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

#[derive(Insertable)]
#[diesel(table_name = groups)]
pub struct NewGroup {
    base_actor_id: i32,
}

impl NewGroup {
    pub fn insert(self, conn: &mut PgConnection) -> Result<Group, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(groups::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn new(base_actor: &BaseActor) -> Self {
        NewGroup {
            base_actor_id: base_actor.id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::*;

    #[test]
    fn create_group() {
        with_connection(|conn| {
            let group_base = make_base_actor(conn)?;
            let group = make_group(conn, &group_base);

            assert!(group.is_ok());

            Ok(())
        })
    }
}
