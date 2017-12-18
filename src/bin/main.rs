#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate failure;
extern crate rocket_contrib;
extern crate serde;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate ring;
extern crate diesel;
extern crate config;

extern crate _aardwolf as aardwolf;

use ring::rand::SystemRandom;
use rocket::Rocket;
use rocket_contrib::Template;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use config::{Config, Environment};
use std::path::PathBuf;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn db_pool(rocket: &Rocket) -> Pool {
    let database_url = rocket.config().get_str("database_url").expect("Must set Database.url in config");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Could not get DB connection pool")
}

fn app(config: config::Config) -> Rocket {
    let mut db_url = String::new();
    match config.get_str("Database.type").unwrap().to_lowercase().as_str() {
        "postgresql" => db_url.push_str("postgres://"),

        // If we reach this case, it's an error.
        // TODO: Handle it.
        _ => {}
    }

    db_url.push_str(config.get_str("Database.username").unwrap().as_str());
    db_url.push(':');
    db_url.push_str(config.get_str("Database.password").unwrap().as_str());
    db_url.push('@');
    db_url.push_str(config.get_str("Database.host").unwrap().as_str());
    db_url.push(':');
    db_url.push_str(config.get_str("Database.port").unwrap().as_str());
    db_url.push('/');
    db_url.push_str(config.get_str("Database.database").unwrap().as_str());

    let c = rocket::Config::build(rocket::config::Environment::Development)
        .address(config.get_str("Listen.address").unwrap())
        .port(config.get::<u16>("Listen.port").unwrap())
        .extra("database_url", db_url.as_str())
        .unwrap();

    let r = rocket::custom(c, true)
        .mount("/api/v1", routes![
            aardwolf::routes::applications::register_application
        ])
        .mount("/", routes![
            aardwolf::routes::auth::sign_up_form,
            aardwolf::routes::auth::sign_in_form,
            aardwolf::routes::auth::sign_up,
            aardwolf::routes::auth::sign_in,
            aardwolf::routes::auth::confirm,
            aardwolf::routes::auth::sign_out,

            aardwolf::routes::app::home,
            aardwolf::routes::app::home_redirect,
        ])
        .attach(Template::fairing())
        .manage(SystemRandom::new());

    // we need an instance of the app to access the config values in Rocket.toml,
    // so we pass it to the db_pool function, get the pool, and _then_ return the instance
    let pool = db_pool(&r);
    r.manage(pool)
}

fn main() {
    // Set defaults
    let mut config = Config::default();
    config.set_default::<&str>("cfg_file", "aardwolf.toml").unwrap();
    config.set_default::<&str>("Listen.address", "127.0.0.1").unwrap();
    config.set_default("Listen.port", 7878).unwrap();

    // Merge environment variables
    config.merge(Environment::with_prefix("aardwolf")).unwrap();

    // Parse and merge arguments

    // Merge config file.
    let cfg_file: PathBuf = PathBuf::from(config.get_str("cfg_file").unwrap());
    config.merge(config::File::with_name(cfg_file.to_str().unwrap())).unwrap();

    app(config).launch();
}
