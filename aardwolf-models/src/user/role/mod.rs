use chrono::DateTime;
use chrono::offset::Utc;

pub mod permission;
pub mod role_permission;
pub mod user_role;

use schema::roles;
use sql_types::Role as RoleSql;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "roles"]
pub struct Role {
    id: i32,
    name: RoleSql,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Role {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> RoleSql {
        self.name
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
