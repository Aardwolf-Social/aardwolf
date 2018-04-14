use chrono::DateTime;
use chrono::offset::Utc;
use diesel;
use diesel::pg::PgConnection;

use base_actor::BaseActor;
use schema::group_base_actors;
use super::Group;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "group_base_actors"]
pub struct GroupBaseActor {
    id: i32,
    group_id: i32,      // foreign key to Group
    base_actor_id: i32, // foreign key to BaseActor
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl GroupBaseActor {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn group_id(&self) -> i32 {
        self.group_id
    }

    pub fn base_actor_id(&self) -> i32 {
        self.base_actor_id
    }
}

#[derive(Insertable)]
#[table_name = "group_base_actors"]
pub struct NewGroupBaseActor {
    group_id: i32,
    base_actor_id: i32,
}

impl NewGroupBaseActor {
    pub fn insert(self, conn: &PgConnection) -> Result<GroupBaseActor, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(group_base_actors::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn new(group: &Group, base_actor: &BaseActor) -> Self {
        NewGroupBaseActor {
            group_id: group.id(),
            base_actor_id: base_actor.id(),
        }
    }
}
