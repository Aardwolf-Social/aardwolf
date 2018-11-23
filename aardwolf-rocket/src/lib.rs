#![recursion_limit = "128"]
#![feature(custom_derive, proc_macro_hygiene, decl_macro)]

extern crate aardwolf_models;
extern crate aardwolf_templates;
extern crate aardwolf_types;
extern crate bcrypt;
extern crate bs58;
extern crate chrono;
extern crate config;
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_i18n;
extern crate serde;

use aardwolf_templates::Renderable;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use rocket::{
    http::{ContentType, Status},
    request::{self, FromRequest},
    Outcome, Request, Response, Rocket, State,
};
use std::{error::Error, ops::Deref};

#[macro_use]
pub mod action;
pub mod routes;
pub mod session;
pub mod types;

pub fn render_template<R>(r: R) -> Response<'static>
where
    R: Renderable,
{
    let mut buf = Vec::new();

    match r.render(&mut buf) {
        Ok(_) => Response::build()
            .header(ContentType::HTML)
            .sized_body(std::io::Cursor::new(buf))
            .finalize(),
        Err(e) => Response::build()
            .status(Status::InternalServerError)
            .header(ContentType::Plain)
            .sized_body(std::io::Cursor::new(format!("{}", e)))
            .finalize(),
    }
}

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

    let mut routes = routes![routes::app::home, routes::app::home_redirect,];

    #[cfg(debug_assertions)]
    routes.extend(routes![
        // webroot/favicon
        routes::app::webroot,
        // emoji
        routes::app::emoji,
        // themes
        routes::app::themes,
        // styles
        routes::app::stylesheets,
    ]);

    let auth = routes![
        routes::auth::sign_up_form,
        routes::auth::sign_in_form,
        routes::auth::sign_up,
        routes::auth::sign_in,
        routes::auth::confirm,
        routes::auth::sign_out,
        routes::auth::already_signed_out,
    ];

    let personas = routes![
        routes::personas::new,
        routes::personas::create,
        routes::personas::delete,
        routes::personas::switch,
    ];

    let r = rocket::custom(c)
        .mount("/auth", auth)
        .mount("/personas", personas)
        .mount(
            "/api/v1",
            routes![routes::applications::register_application],
        )
        .mount("/", routes)
        // TODO: domain and languages should be config'd
        .manage(rocket_i18n::i18n("aardwolf", vec!["en", "pl"]));

    // we need an instance of the app to access the config values in Rocket.toml,
    // so we pass it to the db_pool function, get the pool, and _then_ return the instance
    let pool = db_pool(&r)?;
    Ok(r.manage(pool))
}

pub fn run(config: config::Config, db_url: String) -> Result<(), Box<dyn Error>> {
    app(config, db_url)?.launch();
    Ok(())
}
