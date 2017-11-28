use uuid::Uuid;
use chrono::NaiveDateTime;

use schema::fedibook::users;

#[derive(Queryable)]
pub(crate) struct User {
    id: Uuid,
    email: String,
    encrypted_password: String,
    account_id: Uuid,
    admin: bool,
    disabled: bool,
    unconfirmed_email: String,
    confirmation_token: String,
    confirmed_at: NaiveDateTime,
    confirmation_sent_at: NaiveDateTime,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name="users"]
pub(crate) struct NewUser {
    encrypted_password: String,
    account_id: Uuid,
    unconfirmed_email: String,
    confirmation_token: String,
    confirmed_at: NaiveDateTime,
    confirmation_sent_at: NaiveDateTime,
}
