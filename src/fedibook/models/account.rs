use uuid::Uuid;
use chrono::NaiveDateTime;

use schema::fedibook::accounts;

#[derive(Queryable, Identifiable)]
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
