use uuid::Uuid;
use chrono::NaiveDateTime;

use schema::aardwolf::accounts;

#[derive(Queryable, Identifiable)]
#[allow(unused_must_use, dead_code)]
pub(crate) struct Account {
    pub id: Uuid,
    username: String,
    domain: Option<String>,
    display_name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="accounts"]
pub(crate) struct NewAccount {
    pub username: String,
}
