use uuid::Uuid;
use chrono::{Utc, NaiveDateTime};

use schema::fedibook::users;

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
pub(crate) struct User {
    id: Uuid,
    email: String,
    encrypted_password: String,
    account_id: Uuid,
    admin: bool,
    disabled: bool,
    pub unconfirmed_email: String,
    confirmation_token: Vec<u8>,
    confirmed_at: Option<NaiveDateTime>,
    confirmation_sent_at: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl User {
    pub(crate) fn confirm(&mut self) -> &mut Self {
        self.email = self.unconfirmed_email.clone();
        self.confirmed_at = Some(Utc::now().naive_utc());
        self
    }
}

#[derive(Insertable)]
#[table_name="users"]
pub(crate) struct NewUser {
    pub encrypted_password: String,
    pub account_id: Uuid,
    pub unconfirmed_email: String,
    pub confirmation_token: Vec<u8>,
    pub confirmation_sent_at: NaiveDateTime,
}
