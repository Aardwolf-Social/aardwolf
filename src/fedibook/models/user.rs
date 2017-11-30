use std::sync::Arc;

use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::outcome::IntoOutcome;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::{Utc, NaiveDateTime};

use {DbConn, Pool};
use models::account::Account;
use schema::fedibook::users;

#[derive(Queryable, Identifiable, AsChangeset, Associations, Debug)]
#[belongs_to(Account, foreign_key = "account_id")]
pub(crate) struct User {
    pub id: Uuid,
    pub email: String,
    pub encrypted_password: String,
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

    pub(crate) fn get(id: &str, db: &PgConnection) -> Option<User> {
        use schema::fedibook::users::dsl::*;
        users.find(id).first(db).ok()
    }
}

impl<'l, 'r> FromRequest<'l, 'r> for User {
    type Error = ();

    fn from_request(request: &'l Request<'r>) -> request::Outcome<Self, Self::Error> {

        let pool = request.guard::<State<Pool>>()?;
        let db = match pool.get() {
            Ok(p) => p,
            Err(_) => return Outcome::Failure((Status::ServiceUnavailable, ()))
        };
        let user_id = request.cookies()
            .get_private("user_id")
            .and_then(|c| Some(c.value().to_string()));
        let user = user_id.and_then(|id| User::get(&id, &db));
        user.or_forward(())
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
