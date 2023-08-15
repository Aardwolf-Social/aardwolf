use std::env;
use anyhow::{Context, Result};
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use config::{Config, Environment, File, FileFormat};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'c', long, help = "Sets a custom config file")]
    config: Option<std::path::PathBuf>,

    #[arg(short = 'l', long, help = "Sets logging destination")]
    log: Option<std::path::PathBuf>,

    #[command(flatten)]
    verbose: Verbosity,
}

pub fn configure(parsed_args: Args) -> Result<Config> {
    // Set defaults
    let mut builder = Config::builder();
    builder = builder
        .set_default("cfg_file", concat!(env!("CARGO_PKG_NAME"), ".toml"))
        .context(ErrorKind::ConfigImmutable)?;
    builder = builder
        .set_default("Log.file", "_CONSOLE_")
        .context(ErrorKind::ConfigImmutable)?;
    builder = builder
        .set_default("Web.address", "127.0.0.1")
        .context(ErrorKind::ConfigImmutable)?;
    builder = builder
        .set_default("Web.port", 7878)
        .context(ErrorKind::ConfigImmutable)?;

    // Determine config file
    let config_path = parsed_args.config.unwrap();
    let config_file = config_path.to_str().unwrap();

    builder = builder
        .set_override("cfg_file", config_file)
        .context(ErrorKind::ConfigImmutable)?;

    builder = builder.add_source(File::new(config_file, FileFormat::Toml)); // Or whatever format

        builder = builder
            .set_override("cfg_file", config_file)
            .context(ErrorKind::ConfigImmutable)?;

    // Apply environmenet variable overrides
    let env_vars = Environment::with_prefix("AARDWOLF")
        .separator("_")
        .ignore_empty(true);
    builder = builder.add_source(env_vars);

    let log_path = parsed_args.log.unwrap(); 
    builder = builder
        .set_override("Log.file", log_path.to_str())
        .context(ErrorKind::ConfigImmutable)?;
    
    match builder.build() {
        Ok(config) => {
            env::set_var("DATABASE_URL", db_conn_string(&config)?);
            return Ok(config)
        },
        Err(e) => {
            // Throw an error
            return Err(e.into());
        }
    }
}

pub fn db_conn_string(config: &Config) -> Result<String> {
    let keys = [
        "Database.type",
        "Database.username",
        "Database.password",
        "Database.host",
        "Database.port",
        "Database.database",
    ];

    let string_vec: Vec<String> = keys
        .iter()
        .map(|key| config.get_string(key))
        .collect::<Result<_, _>>()
        .context(ErrorKind::ConfigMissingKeys)?;

    match string_vec[0].as_str()
        {
            "postgres" | "postgresql" => (),
            _ => Err(ErrorKind::UnsupportedDbScheme)?,
        }

    Ok(format!(
        "{scheme}://{username}:{password}@{host}:{port}/{database}",
        scheme = string_vec[0],
        username = string_vec[1],
        password = string_vec[2],
        host = string_vec[3],
        port = string_vec[4],
        database = string_vec[5],
    ))
}

#[derive(Debug, thiserror::Error)]
#[error("Configuration was missing expected keys: [{:?}]", _0)]
pub struct MissingKeys(Vec<String>);

#[derive(Clone, Copy, Debug, Eq, thiserror::Error, Hash, PartialEq)]
pub enum ErrorKind {
    #[error("Unsupported database scheme, only 'postgres' and 'postgresql' are allowed.")]
    UnsupportedDbScheme,
    #[error("Configuration was missing expected keys")]
    ConfigMissingKeys,
    #[error("Config struct cannot be modified")]
    ConfigImmutable,
}

#[cfg(feature = "simple-logging")]
pub fn begin_log(config: &config::Config) {
    use log::LevelFilter;

    match config.get_string("Log.file").unwrap().as_ref() {
        "_CONSOLE_" => (),
        l => simple_logging::log_to_file(l, LevelFilter::Debug).unwrap(),
    }
}

#[cfg(feature = "syslog")]
pub fn begin_log(config: &config::Config) {
    // TODO: Implement log-syslog:begin_log()
}

#[cfg(feature = "systemd")]
pub fn begin_log(config: &config::Config) {
    // TODO: Implement use-systemd:begin_log()
}
