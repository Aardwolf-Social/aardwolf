#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate clap;
extern crate config;
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate r2d2;
extern crate r2d2_diesel;
// extern crate ring;
extern crate log;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate yaml_rust;
extern crate rocket_i18n;

#[cfg(feature = "log-simple")]
extern crate simple_logging;
#[cfg(feature = "log-syslog")]
extern crate syslog;
#[cfg(feature = "use-systemd")]
extern crate systemd;

extern crate _aardwolf as aardwolf;

mod common;
use common::{configure, db_conn_string};

use failure::Error;
// use ring::rand::SystemRandom;
use clap::App;
use diesel::pg::PgConnection;
use log::LevelFilter;
use r2d2_diesel::ConnectionManager;
use rocket::Rocket;
use rocket_contrib::Template;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn db_pool(rocket: &Rocket) -> Result<Pool, Error> {
    let database_url = rocket.config().get_str("database_url")?;
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Ok(r2d2::Pool::builder().build(manager)?)
}

fn app(config: config::Config) -> Result<Rocket, Error> {
    let db_url = db_conn_string(&config)?;

    let c = rocket::Config::build(rocket::config::Environment::Development)
        .address(config.get_str("Web.Listen.address")?)
        .port(config.get::<u16>("Web.Listen.port")?)
        .extra("database_url", db_url.as_str())
        .unwrap();

    let mut routes = routes![
        aardwolf::routes::app::home,
        aardwolf::routes::app::home_redirect,
    ];

    #[cfg(debug_assertions)]
    routes.extend(routes![
        // webroot/favicon
        aardwolf::routes::app::webroot,
        // emoji
        aardwolf::routes::app::emoji,
        // themes
        aardwolf::routes::app::themes,
    ]);

    let auth = routes![
        aardwolf::routes::auth::sign_up_form,
        aardwolf::routes::auth::sign_up_form_with_error,
        aardwolf::routes::auth::sign_in_form,
        aardwolf::routes::auth::sign_in_form_with_error,
        aardwolf::routes::auth::sign_up,
        aardwolf::routes::auth::sign_in,
        aardwolf::routes::auth::confirm,
        aardwolf::routes::auth::sign_out,
    ];

    let personas = routes![
        aardwolf::routes::personas::new,
        aardwolf::routes::personas::create,
        aardwolf::routes::personas::delete,
        aardwolf::routes::personas::switch,
    ];

    let r = rocket::custom(c, true)
        .mount("/auth", auth)
        .mount("/personas", personas)
        .mount(
            "/api/v1",
            routes![aardwolf::routes::applications::register_application],
        )
        .mount("/", routes)
        .attach(Template::fairing())//;
    // .manage(SystemRandom::new());

        // Just for giggles, what happens if I put the rocket_i18n fairing here....

        // Register the fairing. The parameter is the domain you want to use (the name of your app most of the time)
        .attach(rocket_i18n::I18n::new("my_app"))
        // Eventually register the Tera filters (only works with the master branch of Rocket)
        .attach(rocket_contrib::Template::custom(|engines| {
                rocket_i18n::tera(&mut engines.tera);
        }));

    // we need an instance of the app to access the config values in Rocket.toml,
    // so we pass it to the db_pool function, get the pool, and _then_ return the instance
    let pool = db_pool(&r)?;
    Ok(r.manage(pool))
}

fn cli<'a, 'b>(yaml: &'a yaml_rust::yaml::Yaml) -> App<'a, 'b> {
    App::from_yaml(yaml)
        .name(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
}

#[cfg(feature = "log-simple")]
fn begin_log(config: &config::Config) {
    match config.get_str("log_file").unwrap().as_ref() {
        "_CONSOLE_" => (),
        l => simple_logging::log_to_file(l, LevelFilter::Info).unwrap(),
    }
}

#[cfg(feature = "log-syslog")]
fn begin_log(config: &config::Config) {
    // TODO: Implement log-syslog:begin_log()
}

#[cfg(feature = "use-systemd")]
fn begin_log(config: &config::Config) {
    // TODO: Implement use-systemd:begin_log()
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let cli = cli(&yaml);
    let config = configure(cli).unwrap();
    begin_log(&config);

    app(config).unwrap().launch();
}
