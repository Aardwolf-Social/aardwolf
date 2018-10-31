#![recursion_limit = "128"]
#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate aardwolf_models;
extern crate aardwolf_types;
extern crate bcrypt;
extern crate bs58;
extern crate chrono;
#[macro_use]
extern crate collection_macros;
extern crate config;
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request, Rocket, State,
};
use rocket_contrib::Template;
use std::{error::Error, ops::Deref};

pub mod routes;
pub mod types;

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

fn db_pool(rocket: &Rocket) -> Result<Pool, Box<dyn Error>> {
    let database_url = rocket.config().get_str("database_url")?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Ok(r2d2::Pool::builder().build(manager)?)
}

fn app(config: config::Config, db_url: String) -> Result<Rocket, Box<dyn Error>> {
    let c = rocket::Config::build(rocket::config::Environment::Development)
        .address(config.get_str("Web.Listen.address")?)
        .port(config.get::<u16>("Web.Listen.port")?)
        .extra("database_url", db_url.as_str())
        .unwrap();

    let mut routes = routes![self::routes::app::home, self::routes::app::home_redirect,];

    #[cfg(debug_assertions)]
    routes.extend(routes![
        // webroot/favicon
        self::routes::app::webroot,
        // emoji
        self::routes::app::emoji,
        // themes
        self::routes::app::themes,
    ]);

    let auth = routes![
        self::routes::auth::sign_up_form,
        self::routes::auth::sign_up_form_with_error,
        self::routes::auth::sign_in_form,
        self::routes::auth::sign_in_form_with_error,
        self::routes::auth::sign_up,
        self::routes::auth::sign_in,
        self::routes::auth::confirm,
        self::routes::auth::sign_out,
    ];

    let personas = routes![
        self::routes::personas::new,
        self::routes::personas::create,
        self::routes::personas::delete,
        self::routes::personas::switch,
    ];

    let r = rocket::custom(c, true)
        .mount("/auth", auth)
        .mount("/personas", personas)
        .mount(
            "/api/v1",
            routes![self::routes::applications::register_application],
        )
        .mount("/", routes)
        .attach(Template::fairing());
    // .manage(SystemRandom::new());

    // we need an instance of the app to access the config values in Rocket.toml,
    // so we pass it to the db_pool function, get the pool, and _then_ return the instance
    let pool = db_pool(&r)?;
    Ok(r.manage(pool))
}

pub fn run(config: config::Config, db_url: String) -> Result<(), Box<dyn Error>> {
    app(config, db_url)?.launch();
    Ok(())
}
