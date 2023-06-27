use std::error::Error as _;

use aardwolf::{begin_log, configure};
use clap::{load_yaml, App};
use config::Config;
use toml::from_str;

fn cli(toml: &str) -> App {
    let table: toml::value::Table = toml.parse().unwrap();
    App::from(table)
        .name(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
}

pub trait AardwolfServer {
    fn run(&self, config: &Config, db_url: &str) -> Result<(), Box<dyn Error>>;
}

#[cfg(feature = "actix")]
mod actix {
    use config::Config;
    use std::error::Error;

    use super::AardwolfServer;

    pub struct Server;

    impl AardwolfServer for Server {
        fn run(&self, config: &Config, db_url: &str) -> Result<(), Box<dyn Error>> {
            aardwolf_actix::run(config, db_url)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let toml = include_str!("cli.toml");
    let cli = cli(toml);
    let config = configure(cli)?;
    let db_url = aardwolf::db_conn_string(&config)?;

    begin_log(&config);

    #[cfg(feature = "actix")]
    let server = actix::Server;

    server.run(&config, &db_url)?;

    Ok(())
}
