use std::path;

extern crate libxml;
use self::libxml::parser::*;
use self::libxml::tree::*;

// TODO: Implement Errors
pub enum Error {

}

pub struct Config {
    pub file: path::PathBuf,
    pub database: Database,
}

pub struct Database {
    pub username: String,
    pub password: String,
    pub address: String,
    pub port: u32,
}

impl Config {
    // TODO: Change return type to Result<Config, Error> after implementing Error.
    // TODO: Add error handling.
    pub fn new( cfg_file: path::PathBuf) -> Result<Config, String> {  
        let parser = Parser::default();
        let root = parser
            .parse_file(
                cfg_file
                .to_str()
                .unwrap()
            )
            .unwrap()
            .get_root_element();
        
        let mut cfg = Config {
            file: cfg_file,
            database: Database {
                username: "".to_string(),
                password: "".to_string(),
                address: "".to_string(),
                port: 0,
            }
        };
        
        cfg.parse(&root);

        Ok(cfg)
    }

    // TODO: Recursively parse node data into config values.
    fn parse(&mut self, node: &Node) {

    }
}
