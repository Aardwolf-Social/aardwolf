use chrono::DateTime;
use chrono::offset::Utc;
use diesel;
use diesel::pg::PgConnection;

pub mod group_base_actor;

use base_actor::BaseActor;
use schema::groups;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "groups"]
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
}

#[derive(Insertable)]
#[table_name = "groups"]
pub struct NewGroup {
    base_actor_id: i32,
}

impl NewGroup {
    pub fn insert(self, conn: &PgConnection) -> Result<Group, diesel::result::Error> {
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
