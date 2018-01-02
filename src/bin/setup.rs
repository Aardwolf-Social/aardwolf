#[macro_use] extern crate clap;
extern crate config;

use std::error::Error as StdError;
use std::io::ErrorKind;
use std::process::{self, Command};

use clap::App;

mod common;

fn main() {
    let yaml = load_yaml!("setup.yml");
    let app = App::from_yaml(yaml)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"));
    let config = common::configure(app);
    let db_url = common::db_conn_str(&config);
    println!("using database url `{}' to setup the aardwolf database", &db_url);
    let output = Command::new("diesel")
        .arg("setup")
        .env("DATABASE_URL", &db_url)
        .output();
    if let Err(e) = output {
        match e.kind() {
            ErrorKind::NotFound => {
                eprintln!("Could not find `diesel` binary, please use `cargo install diesel_cli` to install it");
            },
            _ => eprintln!("got error {}", e.description()),
        }
        process::exit(255);
    }
    println!("database successfully set up, running migrations");
    let output = Command::new("diesel")
        .arg("migration")
        .arg("run")
        .env("DATABASE_URL", &db_url)
        .output();
    if let Err(e) = output {
        eprintln!("got error {}", e.description());
        process::exit(255);
    }
    println!("database migrations were successfully run, you're ready to go!");
}

