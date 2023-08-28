use std::env;
use std::io::ErrorKind;
use anyhow::{Context, Result};
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use config::{Config, Environment, File, FileFormat};
use log::LevelFilter;
use std::path::PathBuf;

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

pub fn configure(parsed_args: Args) -> Result<Config> {
    // Set defaults
    let mut builder = Config::builder()
        .set_default("cfg_file", concat!(env!("CARGO_PKG_NAME"), ".toml"))?
        .set_default("Log.file", "_CONSOLE_")?
        .set_default("Web.address", "127.0.0.1")?
        .set_default("Web.port", 7878)?
        .set_default("Database.type", "postgres")?;

    // Determine config file
    let config_path = parsed_args.config.ok_or_else(|| anyhow::anyhow!("No config path provided"))?;
    let config_file = config_path.to_str().ok_or_else(|| anyhow::anyhow!("Failed to convert config path to string"))?;

    builder = set_config_file_override(builder, config_file)?;
    builder = builder.add_source(File::new(config_file, FileFormat::Toml));
    builder = builder
        .merge(create_override_config(config_file)?)
        .context(ErrorKind::ConfigImmutable)?;

    // Apply environment variable overrides
    let env_prefix = env!("CARGO_PKG_NAME").to_ascii_uppercase().replace_char('-', '_');
    let env_vars = Environment::with_prefix(env_prefix)
        .separator("_")
        .ignore_empty(true);
    builder = builder.add_source(env_vars);

    let log_path = parsed_args.log.ok_or_else(|| anyhow::anyhow!("No log path provided"))?;
    builder = builder
        .merge(create_log_override_config(log_path.to_str())?)
        .context(ErrorKind::ConfigImmutable)?;

    match builder.build() {
        Ok(config) => {
            let db_url = db_conn_string(&config)?;
            // Pass db_url directly to the function that needs it
            some_function_needs_db_url(db_url)?;
            Ok(config)
        },
        Err(e) => Err(e.into()),
    }
}

fn create_override_config(config_file: &str) -> Result<Config> {
    let mut overrides = Config::default();
    overrides.set("cfg_file", config_file)?;
    Ok(overrides)
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

    let supported_schemes = ["postgres", "postgresql", "mysql", "sqlite"];
    if !supported_schemes.contains(&string_vec[0].as_str()) {
        Err(ErrorKind::UnsupportedDbScheme)?;
    }

    Ok(concat!(
        string_vec[0], "://", string_vec[1], ":", string_vec[2], "@
        string_vec[0], "://", string_vec[1], ":", string_vec[2], "@", string_vec[3], ":", string_vec[4], "/", string_vec[5],
    ).to_string())
}

#[cfg(feature = "simple-logging")]
pub fn begin_log(config: &config::Config, level: LevelFilter) -> Result<(), Box<dyn std::error::Error>> {
    match config.get_string("Log.file")?.as_ref() {
        "_CONSOLE_" => Ok(()),
        l => {
            simple_logging::log_to_file(l, level)?;
            Ok(())
        },
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
