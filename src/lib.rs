use std::{env, fmt};

use anyhow::{Context, Result};
use clap::App;
use config::{Config, ConfigError, Environment};

pub fn configure(app: App) -> Result<Config> {
    // Order of how configuration values are set:
    // cli arguments > environment variables > config file > default values

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
    // TODO: Is there a better way to handle this?
    if let Ok(c) = env::var("AARDWOLF_CONFIG") {
        config
            .set("cfg_file", c)
            .context(ErrorKind::ConfigImmutable)?;
    }

    // Parse arguments
    let args = app.get_matches();

    if let Some(c) = args.value_of("config") {
        config
            .set("cfg_file", c)
            .context(ErrorKind::ConfigImmutable)?;
    }

    // Merge config file and apply over-rides
    let cfg_file_string = config
        .get_str("cfg_file")
        .context(ErrorKind::ConfigMissingKeys)?;
    let cfg_file = config::File::with_name(&cfg_file_string);
    config.merge(cfg_file).context(ErrorKind::ConfigImmutable)?;

    // Apply environment variable overrides
    let env_vars = Environment::with_prefix("AARDWOLF")
        .separator("_")
        .ignore_empty(true);
    config.merge(env_vars).context(ErrorKind::ConfigImmutable)?;

    // Remove the need for a .env file to avoid defining env vars twice.
    env::set_var("DATABASE_URL", db_conn_string(&config)?);

    if let Some(l) = args.value_of("log") {
        config
            .set("Log.file", l)
            .context(ErrorKind::ConfigImmutable)?;
    }

    Ok(config)
}

pub fn db_conn_string(config: &Config) -> Result<String> {
    let keys = vec![
        "Database.type",
        "Database.username",
        "Database.password",
        "Database.host",
        "Database.port",
        "Database.database",
    ];

    let (string_vec, error_vec) = keys.into_iter().map(|key| config.get_str(key)).fold(
        (Vec::new(), Vec::new()),
        |(mut string_vec, mut error_vec), res| {
            match res {
                Ok(string) => string_vec.push(string),
                Err(error) => {
                    if let ConfigError::NotFound(key) = error {
                        error_vec.push(key);
                    }
                }
            }

            (string_vec, error_vec)
        },
    );

    if !error_vec.is_empty() {
        return Err(ErrorKind::ConfigMissingKeys).context(MissingKeys(error_vec));
    }

    match string_vec[0].as_ref() {
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

    match config.get_str("Log.file").unwrap().as_ref() {
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
