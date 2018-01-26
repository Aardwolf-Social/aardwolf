use std::env;
use std::error::Error as StdError;
use std::fmt;

use clap::App;
use config::{self, Config, ConfigError};
use r2d2::Error as R2D2Error;
use rocket;

pub fn configure(app: App) -> Result<Config, Error> {
    // Set defaults
    let mut config = Config::default();
    config.set_default::<&str>("cfg_file", concat!(env!("CARGO_PKG_NAME"), ".toml"))?;
    config.set_default::<&str>("log_file", concat!(env!("CARGO_PKG_NAME"), ".log"))?;
    config.set_default::<&str>("Web.Listen.address", "127.0.0.1")?;
    config.set_default("Web.Listen.port", 7878)?;

    // Parse arguments
    let args = app.get_matches();

    // Determine config file
    // TODO: Is there a better way to handle this?
    if let Ok(c) = env::var("AARDWOLF_CONFIG") {
        config.set("cfg_file", c)?;
    }

    if let Some(c) = args.value_of("config") {
        config.set("cfg_file", c)?;
    }

    // Merge config file and apply over-rides
    let cfg_file_string = config.get_str("cfg_file")?;
    let cfg_file = config::File::with_name(&cfg_file_string);
    config.merge(cfg_file)?;

    //  TODO: Is there a better way to handle this?
    if let Ok(l) = env::var("AARDWOLF_LOG") {
        config.set("log_file", l)?;
    }

    if let Some(l) = args.value_of("log") {
        config.set("log_file", l)?;
    }

    Ok(config)
}

pub fn db_conn_str(config: &Config) -> Result<String, Error> {
    Ok(DatabaseSetupOptions::new(config).to_database_setup()?.to_string())
}

#[derive(Debug)]
pub struct GetError(String);

impl fmt::Display for GetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find required key: {}", self.0)
    }
}

impl StdError for GetError {
    fn description(&self) -> &str {
        "Could not find a required key"
    }
}

#[derive(Debug)]
pub enum Error {
    R2D2(R2D2Error),
    RocketConfig(rocket::config::ConfigError),
    UnsupportedDbScheme(String),
    Config(ConfigError),
    ConfigMissingKeys(Vec<String>),
    ConfigImmutable,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::R2D2(ref r2_err) => r2_err.fmt(f),
            Error::RocketConfig(ref rc) => rc.fmt(f),
            Error::UnsupportedDbScheme(ref scheme) => write!(f, "Unsupported db scheme {}, only 'postgres' and 'postgresql' are supported", scheme),
            Error::Config(ref cfg_err) => cfg_err.fmt(f),
            Error::ConfigMissingKeys(ref keys) => write!(f, "Could not find required keys: [{}]", keys.join(", ")),
            Error::ConfigImmutable => write!(f, "Config object is immutable"),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::R2D2(_) => "Error when interacting with db pool",
            Error::RocketConfig(_) => "Error configuring Rocket",
            Error::UnsupportedDbScheme(_) => "Unsupported db scheme, only 'postgres' and 'postgresql' are supported",
            Error::Config(ref cfg_err) => cfg_err.description(),
            Error::ConfigMissingKeys(_) => "Could not find required keys",
            Error::ConfigImmutable => "Config object is immutable",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::R2D2(ref r2_err) => Some(r2_err),
            Error::RocketConfig(ref rc) => Some(rc),
            Error::Config(ref cfg_err) => Some(cfg_err),
            _ => None,
        }
    }
}

impl From<ConfigError> for Error {
    fn from(e: ConfigError) -> Self {
        match e {
            ConfigError::Frozen => Error::ConfigImmutable,
            ConfigError::NotFound(key) => GetError(key).into(),
            other => Error::Config(other),
        }
    }
}

impl From<GetError> for Error {
    fn from(error: GetError) -> Self {
        Error::ConfigMissingKeys(vec![error.0])
    }
}

impl From<Vec<GetError>> for Error {
    fn from(errors: Vec<GetError>) -> Self {
        let keys = errors.into_iter().fold(Vec::new(), |mut acc, err| {
            acc.push(err.0);

            acc
        });

        Error::ConfigMissingKeys(keys)
    }
}

impl From<R2D2Error> for Error {
    fn from(e: R2D2Error) -> Self {
        Error::R2D2(e)
    }
}

impl From<rocket::config::ConfigError> for Error {
    fn from(e: rocket::config::ConfigError) -> Self {
        Error::RocketConfig(e)
    }
}

struct DatabaseSetupOptions(Vec<Result<String, ConfigError>>);

impl DatabaseSetupOptions {
    fn new(config: &Config) -> Self {
        let keys = vec!["Database.type", "Database.username", "Database.password", "Database.host", "Database.port", "Database.database"];

        DatabaseSetupOptions(keys.into_iter().map(|key| config.get_str(key)).collect())
    }

    fn to_database_setup(self) -> Result<DatabaseSetup, Error> {
        let (string_vec, error_vec) = self.0.into_iter().fold((Vec::new(), Vec::new()), |(mut string_vec, mut error_vec), res| {
            match res {
                Ok(string) => string_vec.push(string),
                Err(error) => match error {
                    ConfigError::NotFound(key) => error_vec.push(GetError(key)),
                    _ => (),
                }
            }

            (string_vec, error_vec)
        });

        if !error_vec.is_empty() {
            return Err(error_vec.into())
        }

        let db_setup = DatabaseSetup {
            scheme: string_vec[0].clone(),
            username: string_vec[1].clone(),
            password: string_vec[2].clone(),
            host: string_vec[3].clone(),
            port: string_vec[4].clone(),
            database: string_vec[5].clone(),
        };

        match db_setup.scheme.as_ref() {
            "postgres" | "postgresql" => (),
            kind => return Err(Error::UnsupportedDbScheme(kind.to_owned())),
        }

        Ok(db_setup)
    }
}

struct DatabaseSetup {
    scheme: String,
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}

impl DatabaseSetup {
    fn to_string(&self) -> String {
        format!("{scheme}://{username}:{password}@{host}:{port}/{database}",
                scheme=self.scheme,
                username=self.username,
                password=self.password,
                host=self.host,
                port=self.port,
                database=self.database)
    }
}
