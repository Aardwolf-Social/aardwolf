use chrono::{offset::Utc, DateTime};

use schema::user_roles;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "user_roles"]
pub struct UserRole {
    id: i32,
    user_id: i32, // foreign key to User
    role_id: i32, // foreign key to Role
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl UserRole {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn role_id(&self) -> i32 {
        self.role_id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}
