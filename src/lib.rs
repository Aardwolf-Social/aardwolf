use std::env;
use anyhow::{Context, Result};
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use config::{Config, ConfigError, Environment};

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
    let mut config = Config::default();
    config
        .set_default::<&str>("cfg_file", concat!(env!("CARGO_PKG_NAME"), ".toml"))
        .context(ErrorKind::ConfigImmutable)?;
    config
        .set_default::<&str>("Log.file", "_CONSOLE_")
        .context(ErrorKind::ConfigImmutable)?;
    config
        .set_default::<&str>("Web.address", "127.0.0.1")
        .context(ErrorKind::ConfigImmutable)?;
    config
        .set_default("Web.port", 7878)
        .context(ErrorKind::ConfigImmutable)?;

    // Determine config file
    if let Ok(config_file) = env::var("AARDWOLF_CONFIG") {
        config
            .set("cfg_file", config_file)
            .context(ErrorKind::ConfigImmutable)?;
    }

    if let Some(config_path) = parsed_args.config {
        config
            .set("cfg_file", config_path.to_str())
            .context(ErrorKind::ConfigImmutable)?;
    }

    // Merge config file and apply overrides
    let config_file_string = config
        .get_string("cfg_file")
        .context(ErrorKind::ConfigMissingKeys)?;
    let config_file = config::File::with_name(&config_file_string);
    config.merge(config_file).context(ErrorKind::ConfigImmutable)?;

    // Apply environment variable overrides
    let env_vars = Environment::with_prefix("AARDWOLF")
        .separator("_")
        .ignore_empty(true);
    config.merge(env_vars).context(ErrorKind::ConfigImmutable)?;

    // Remove the need for a .env file to avoid defining env vars twice.
    env::set_var("DATABASE_URL", db_conn_string(&config)?);

    if let Some(log_path) = parsed_args.log {
        config
            .set("Log.file", log_path.to_str())
            .context(ErrorKind::ConfigImmutable)?;
    }

    Ok(config)
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
