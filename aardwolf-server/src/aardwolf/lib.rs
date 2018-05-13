#![recursion_limit = "128"]
#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate aardwolf_models;
extern crate bcrypt;
extern crate bs58;
extern crate chrono;
#[macro_use]
extern crate collection_macros;
#[macro_use]
extern crate derive_builder;
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate ring;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate url;
extern crate url_serde;
extern crate uuid;

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use r2d2_diesel::ConnectionManager;
use diesel::pg::PgConnection;

pub mod controllers;
pub mod forms;
pub mod types;
pub mod routes;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl<'l, 'r> FromRequest<'l, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'l Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
