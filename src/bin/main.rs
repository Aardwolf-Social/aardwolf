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
    let database_url = rocket.config().get_str("database_url").expect("Must set DATABASE_URL");
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(config, manager).expect("Could not get DB connection pool")
}

fn app() -> Rocket {
    let r = rocket::ignite()
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
    config.set_default::<&str>("cfg_file", "/etc/aardwolf/config.toml").unwrap();

    // Merge environment variables
    config.merge(Environment::with_prefix("aardwolf")).unwrap();

    // Parse and merge arguments

    // Merge config file.
    let cfg_file: PathBuf = PathBuf::from(config.get_str("cfg_file").unwrap());
    config.merge(config::File::with_name(cfg_file.to_str().unwrap())).unwrap();

    app().launch();
}
