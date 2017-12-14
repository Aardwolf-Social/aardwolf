#![recursion_limit="128"]
#![feature(try_from)]
#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate bs58;
extern crate bcrypt;
extern crate chrono;
extern crate ring;
extern crate rocket;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket_contrib;
extern crate serde;
extern crate uuid;
#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate derive_builder;
#[macro_use] extern crate collection_macros;
#[macro_use] extern crate diesel;

use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use r2d2_diesel::ConnectionManager;
use diesel::pg::PgConnection;

pub mod models;
pub mod controllers;
pub mod forms;
pub mod routes;
pub mod schema;

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
