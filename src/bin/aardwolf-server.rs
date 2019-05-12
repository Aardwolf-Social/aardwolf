use std::error::Error;

use aardwolf::{begin_log, configure};
use clap::{load_yaml, App};
use config::Config;

fn cli<'a, 'b>(yaml: &'a yaml_rust::yaml::Yaml) -> App<'a, 'b> {
    App::from_yaml(yaml)
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
    let yaml = load_yaml!("cli.yml");
    let cli = cli(&yaml);
    let config = configure(cli)?;
    let db_url = aardwolf::db_conn_string(&config)?;

    begin_log(&config);

    #[cfg(feature = "actix")]
    let server = actix::Server;

    server.run(&config, &db_url)?;

    Ok(())
}
