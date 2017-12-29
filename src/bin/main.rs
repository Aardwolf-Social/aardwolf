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
#[macro_use]
extern crate clap;

extern crate _aardwolf as aardwolf;

use ring::rand::SystemRandom;
use rocket::Rocket;
use rocket_contrib::Template;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use config::Config;
use clap::App;
use std::path::PathBuf;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn db_pool(rocket: &Rocket) -> Pool {
    let database_url = rocket.config().get_str("database_url").expect("Must set Database.url in config");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Could not get DB connection pool")
}

fn app(config: config::Config) -> Rocket {
    let mut db_url = String::new();
    match config.get_str("Database.type").unwrap().to_lowercase().as_str() {
        "postgresql" | "postgres" => db_url.push_str("postgres://"),

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
        .address(config.get_str("Web.Listen.address").unwrap())
        .port(config.get::<u16>("Web.Listen.port").unwrap())
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

            // Adding route to / 
            aardwolf::routes::app::index,
        ])
        .attach(Template::fairing())
        .manage(SystemRandom::new());

    // we need an instance of the app to access the config values in Rocket.toml,
    // so we pass it to the db_pool function, get the pool, and _then_ return the instance
    let pool = db_pool(&r);
    r.manage(pool)
}

fn configure() -> Config {
    // Set defaults
    let mut config = Config::default();
    config.set_default::<&str>("cfg_file", concat!(env!("CARGO_PKG_NAME"), ".toml")).unwrap();
    config.set_default::<&str>("log_file", concat!(env!("CARGO_PKG_NAME"), ".log")).unwrap();
    config.set_default::<&str>("Web.Listen.address", "127.0.0.1").unwrap();
    config.set_default("Web.Listen.port", 7878).unwrap();

    // Parse arguments
    let yaml = load_yaml!("cli.yml");
    let args = App::from_yaml(yaml)
        .name(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    // Determine config file
    // TODO: Is there a better way to handle this?
    match env::var("AARDWOLF_CONFIG") {
        Ok(c) => {config.set("cfg_file", c).unwrap();},
        Err(_) => {}
    }

    match args.value_of("config") {
        Some(c) => {config.set("cfg_file", c).unwrap();},
        None => {}
    }

    // Merge config file and apply over-rides
    let cfg_file: PathBuf = PathBuf::from(config.get_str("cfg_file").unwrap());
    config.merge(config::File::with_name(cfg_file.to_str().unwrap())).unwrap();

    //  TODO: Is there a better way to handle this?
    match env::var("AARDWOLF_LOG") {
        Ok(l) => {config.set("log_file", l).unwrap();},
        Err(_) => {}
    }

    match args.value_of("log") {
        Some(l) => {config.set("log_file", l).unwrap();},
        None => {}
    }

    config
}

fn main() {
    let config = configure();

    app(config).launch();
}
