//-
// THIS IS THE BROKEN VERSION OF ./src/lib.rs
// It was created to use the config::Builder, and does not compile.
//
use std::io::ErrorKind;
use anyhow::{Context, Result};
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use config::{Config, Environment, File, FileFormat};
use log::LevelFilter;
use std::path::PathBuf;
use url::Url;
use thiserror::Error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'c', long, help = "Sets a custom config file")]
    config: Option<PathBuf>,

    #[arg(short = 'l', long, help = "Sets logging destination")]
    log: Option<PathBuf>,

    #[command(flatten)]
    verbose: Verbosity,
}

#[derive(Debug, Error)]
enum ConfigError {
    #[error("No config path provided")]
    NoConfigPath,
    #[error("No log path provided")]
    NoLogPath,
    #[error("Invalid log path provided")]
    InvalidLogPath,
    #[error("Failed to get database connection string")]
    DbConnStringError,
    #[error("Failed to build configuration")]
    ConfigBuildError,
}

pub fn configure(parsed_args: Args) -> Result<(Config, String), ConfigError> {
    let config_path = parsed_args.config.ok_or(ConfigError::NoConfigPath)?;
    let log_path = parsed_args.log.ok_or(ConfigError::NoLogPath)?;

    let config = Config::builder()
        .add_source(File::with_name("default_config.toml", FileFormat::Toml))
        .set_default(Environment::with_prefix("AARDWOLF").separator("__").ignore_empty(true))
        .add_source(File::from(&config_path))
        .with(|builder| {
            let config_file = config_path.to_str().ok_or(ConfigError::NoConfigPath)?;
            builder.add_source(File::new(&config_file, FileFormat::Toml)).map_err(|_| ConfigError::ConfigBuildError)?;
            builder.merge(create_override_config(&config_file)).map_err(|_| ConfigError::ConfigBuildError)?;
            let env_prefix = env!("CARGO_PKG_NAME").to_ascii_uppercase().replace('-', '_');
            let env_vars = Environment::with_prefix(env_prefix).separator("_").ignore_empty(true);
            builder.add_source(env_vars);
            let log_file = log_path.to_str().ok_or(ConfigError::InvalidLogPath)?;
            builder.set_override("Log.file", &log_file).map_err(|_| ConfigError::ConfigBuildError)?;
            Ok(())
        })?
        .build().map_err(|_| ConfigError::ConfigBuildError)?;

    let db_url = db_conn_string(&config)?;
    Ok((config, db_url))
}

fn db_conn_string(config: &Config) -> Result<String> {
    let db_type = config.get_string("Database.type")?;
    let username = config.get_string("Database.username")?;
    let password = config.get_string("Database.password")?;
    let host = config.get_string("Database.host")?;
    let port = config.get_string("Database.port")?;
    let database = config.get_string("Database.database")?;

    let db_url = Url::parse(&format!("{}://{}:{}@{}:{}/{}", db_type, username, password, host, port, database))?;
    Ok(db_url.as_str().to_owned())
}

fn create_override_config(config_file: &str) -> Result<(), ConfigError> {
    let mut overrides = Config::default();
    overrides.set("cfg_file", config_file)?;
    Ok(())
}

pub fn begin_log(config: &config::Config, level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    let log_file = config.get_string("Log.file")?;
    if log_file != "_CONSOLE_" {
        simple_logging::log_to_file(&log_file, level)?;
    }
    Ok(())
}

#[cfg(feature = "simple-logging")]
pub fn begin_log_wrapper(config: &config::Config, level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    begin_log(config, level)
}

#[cfg(feature = "syslog")]
pub fn begin_log_wrapper(config: &config::Config, level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    begin_log(config, level)
}

#[cfg(feature = "systemd")]
pub fn begin_log_wrapper(config: &config::Config, level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    begin_log(config, level)
}   let host = config.get_string("Database.host")?;
    let port = config.get_string("Database.port")?;
    let database = config.get_string("Database.database")?;

    let db_url = Url::parse(&format!("{}://{}:{}@{}:{}/{}", db_type, username, password, host, port, database))?;
    Ok(db_url.as_str().to_owned())
}

pub fn begin_log(config: &config::Config, level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    let log_file = config.get_string("Log.file")?;
    if log_file != "_CONSOLE_" {
        simple_logging::log_to_file(&log_file, level)?;
    }
    Ok(())
}

#[cfg(feature = "simple-logging")]
pub fn begin_log_wrapper(config: &config::Config, level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    begin_log(config, level)
}

#[cfg(feature = "syslog")]
pub fn begin_log_wrapper(config: &config::Config, level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    begin_log(config, level)
}

#[cfg(feature = "systemd")]
pub fn begin_log_wrapper(config: &config::Config, level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    begin_log(config, level)
}
