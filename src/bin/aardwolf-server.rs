use std::error::Error;

use aardwolf::{begin_log, configure, Args};
use clap::Parser;
use config::Config;

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
    let args = Args::parse();
    let config = configure(args)?;
    let db_url = aardwolf::db_conn_string(&config)?;

    begin_log(&config);

    #[cfg(feature = "actix")]
    let server = actix::Server;

    server.run(&config, &db_url)?;

    Ok(())
}
