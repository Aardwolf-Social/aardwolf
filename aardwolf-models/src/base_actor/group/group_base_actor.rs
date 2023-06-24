use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};

use crate::{
    base_actor::{group::Group, BaseActor},
    schema::group_base_actors,
};

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[diesel(table_name = group_base_actors)]
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

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Insertable)]
#[diesel(table_name = group_base_actors)]
pub struct NewGroupBaseActor {
    group_id: i32,
    base_actor_id: i32,
}

impl NewGroupBaseActor {
    pub fn insert(self, conn: &mut PgConnection) -> Result<GroupBaseActor, diesel::result::Error> {
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

#[cfg(test)]
mod tests {
    use crate::test_helper::*;

    #[test]
    fn create_group_base_actor() {
        with_connection(|conn| {
            let base_group = make_base_actor(conn)?;
            let group = make_group(conn, &base_group)?;

            let base_actor = make_base_actor(conn)?;

            let result = make_group_base_actor(conn, &group, &base_actor);

            assert!(result.is_ok());

            Ok(())
        });
    }
}
