#![allow(proc_macro_derive_resolution_fallback)]
use chrono::{offset::Utc, DateTime};

use schema::role_permissions;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "role_permissions"]
pub struct RolePermission {
    id: i32,
    role_id: i32,       // foreign key to Role
    permission_id: i32, // foreign key to Permission
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl RolePermission {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn role_id(&self) -> i32 {
        self.role_id
    }

    pub fn permission_id(&self) -> i32 {
        self.permission_id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
