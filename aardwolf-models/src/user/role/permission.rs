use chrono::{offset::Utc, DateTime};

use crate::{schema::permissions, sql_types::Permission as PermissionSql};

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "permissions"]
pub struct Permission {
    id: i32,
    name: PermissionSql,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Permission {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn name(&self) -> PermissionSql {
        self.name
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
