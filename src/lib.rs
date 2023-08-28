use std::env;
use std::io::ErrorKind;
use anyhow::{Context, Result};
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use config::{Config, Environment, File, FileFormat};
use log::LevelFilter;
use std::path::PathBuf;

/// Command line arguments struct
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

/// Configures the application using the parsed command line arguments
pub fn configure(parsed_args: Args) -> Result<(Config, String)> {
    let config_path = match parsed_args.config {
        Some(path) => path,
        None => {
            eprintln!("No config path provided");
            std::process::exit(1);
        }
    };

    let config = Config::builder()
        .add_default(File::with_name("default_config.toml"))?
        .add_default(Environment::with_prefix("MY_APP"))?
        .add_default(Environment::with_prefix("MY_APP").separator("__"))?
        .add_default(Environment::with_prefix("MY_APP").separator("__").ignore_empty(true))?
        .add_source(File::from(config_path))?
        .with(|builder| {
            let config_file = config_path.to_str().unwrap_or_default();
            set_config_file_override(builder, &config_file)?;
            builder.add_source(File::new(&config_file, FileFormat::Toml))?;
            builder.merge(create_override_config(&config_file)?).context(ErrorKind::ConfigImmutable)?;
            let env_prefix = env!("CARGO_PKG_NAME").to_ascii_uppercase().replace_char('-', '_');
            let env_vars = Environment::with_prefix(env_prefix).separator("_").ignore_empty(true);
            builder.add_source(env_vars);
            let log_path = parsed_args.log.ok_or_else(|| anyhow::anyhow!("No log path provided"))?;
            let log_file = log_path.to_str().unwrap_or_default();
            builder.set_override("Log.file", &log_file).context(ErrorKind::ConfigImmutable)?;
            Ok(())
        })?
        .build()?;

    let db_url = db_conn_string(&config)?;
    Ok((config, db_url))
}

/// Creates an override configuration with the provided config file
fn create_override_config(config_file: &str) -> Result<Config> {
    let mut overrides = Config::default();
    overrides.set("cfg_file", config_file)?;
    Ok(overrides)
}

/// Generates a database connection string based on the provided config
pub fn db_conn_string(config: &Config) -> Result<String> {
    const SUPPORTED_SCHEMES: [&str; 4] = ["postgres", "postgresql", "mysql", "sqlite"];

    let db_config = DatabaseConfig {
        db_type: config.get_string("Database.type")?,
        username: config.get_string("Database.username")?,
        password: config.get_string("Database.password")?,
        host: config.get_string("Database.host")?,
        port: config.get_string("Database.port")?,
        database: config.get_string("Database.database")?,
    };

    if !SUPPORTED_SCHEMES.contains(&db_config.db_type.as_str()) {
        Err(ErrorKind::UnsupportedDbScheme)?;
    }

    Ok(format!("{}://{}:{}@{}:{}/{}", db_config.db_type, db_config.username, db_config.password, db_config.host, db_config.port, db_config.database))
}

struct DatabaseConfig {
    db_type: String,
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}

#[cfg(feature = "simple-logging")]
/// Begins logging based on the provided config and log level
pub fn begin_log(config: &config::Config, level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    let log_file = config.get_string("Log.file")?;
    if log_file != "_CONSOLE_" {
        simple_logging::log_to_file(&log_file, level)?;
    }
    Ok(())
}

#[cfg(feature = "syslog")]
/// Begins logging based on the provided config and log level
pub fn begin_log(config: &config::Config, level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement log-syslog:begin_log()
    let log_file = config.get_string("Log.file")?;
    if log_file != "_CONSOLE_" {
        simple_logging::log_to_file(&log_file, level)?;
    }
    Ok(())
}

#[cfg(feature = "systemd")]
/// Begins logging based on the provided config and log level
pub fn begin_log(config: &config::Config, level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Implement use-systemd:begin_log()
    let log_file = config.get_string("Log.file")?;
    if log_file != "_CONSOLE_" {
        simple_logging::log_to_file(&log_file, level)?;
    }
    Ok(())
}