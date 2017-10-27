use std::env;
use std::path::PathBuf;

mod config;
use config::*;

fn main() {
    // TODO: Parse command-line arguments
    // Set defaults for command line arguments
    let cfg_file = PathBuf::from("settings.xml");

    // Process and shadow defaults.

    // TODO: Complete config module
    let cfg = Config::new(cfg_file).unwrap();

    // TODO: Initialize/Fork

    // Main loop
    let mut run = true;
    while run {

    }
}
