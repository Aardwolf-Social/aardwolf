use std::{
    io::{self, ErrorKind},
    process::{self, Command, Output},
};
use toml::value::Table;
use clap::{App, Arg};

fn check_out(output: &Result<Output, io::Error>) {
    match *output {
        Ok(ref o) if !o.status.success() => {
            eprintln!(
                "got non-zero exit code, output was:\n\tstdout:\n{}\n\tstderr:\n{}",
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            );
            process::exit(255);
        }
        Err(ref e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    eprintln!("Could not find `diesel` binary, please use `cargo install diesel_cli` to install it");
                }
                _ => eprintln!("got error {}", e),
            }
            process::exit(255);
        }
        Ok(_) => {}
    }
}

fn main() {
    let toml_str = include_str!("setup.toml");
    let toml: Table = toml::from_str(toml_str).unwrap();
    
    let app = App::new(toml["name"].as_str().unwrap())
        .version(toml["version"].as_str().unwrap())
        .author(toml["author"].as_str().unwrap());
    
    let config = aardwolf::configure(app).unwrap();
    let db_url = aardwolf::db_conn_string(&config).unwrap();
    println!(
        "using database url `{}' to setup the aardwolf database",
        &db_url
    );
    
    let output = Command::new("diesel")
        .arg("setup")
        .arg("--migration-dir")
        .arg("aardwolf-models/migrations")
        .env("DATABASE_URL", &db_url)
        .output();
    check_out(&output);
    
    println!("database migrations were successfully run, you're ready to go!");
}
