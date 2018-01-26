#[macro_use] extern crate clap;
extern crate config;
extern crate r2d2;
extern crate rocket;

use std::error::Error as StdError;
use std::io::{self, ErrorKind};
use std::process::{self, Command, Output};

use clap::App;

mod common;

fn check_out(output: &Result<Output, io::Error>) {
    match output {
        &Ok(ref o) if !o.status.success() => {
            eprintln!("got non-zero exit code, output was:\n\tstdout:\n{}\n\tstderr:\n{}",
                    String::from_utf8_lossy(&o.stdout),
                    String::from_utf8_lossy(&o.stderr));
            process::exit(255);
        },
        &Err(ref e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    eprintln!("Could not find `diesel` binary, please use `cargo install diesel_cli` to install it");
                },
                _ => eprintln!("got error {}", e.description()),
            }
            process::exit(255);
        },
        &Ok(_) => {},
    }
}

fn main() {
    let yaml = load_yaml!("setup.yml");
    let app = App::from_yaml(yaml)
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"));
    let config = common::configure(app).unwrap();
    let db_url = common::db_conn_str(&config).unwrap();
    println!("using database url `{}' to setup the aardwolf database", &db_url);
    let output = Command::new("diesel")
        .arg("setup")
        .env("DATABASE_URL", &db_url)
        .output();
    check_out(&output);
    println!("database successfully set up, running migrations");
    let output = Command::new("diesel")
        .arg("migration")
        .arg("run")
        .env("DATABASE_URL", &db_url)
        .output();
    check_out(&output);
    println!("database migrations were successfully run, you're ready to go!");
}

