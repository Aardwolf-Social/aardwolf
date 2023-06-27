mod tests {
    use config::ConfigError;
    use std::{error::Error, str::FromStr};

    #[test]
    fn test_main() -> Result<(), Box<dyn Error>> {
        let toml = r#"
        [package]
        name = "test"
        version = "0.1.0"
        authors = ["Author <author@test.com>"]
        description = "Test app"

        [options]
        debug = true
    "#;
        let cli = super::cli(toml);
        assert_eq!(cli.get_name(), Some("test".to_string()));

        let config = super::configure(cli).unwrap();
        assert_eq!(config.get_bool("options.debug")?, true);

        let db_url = super::aardwolf::db_conn_string(&config)?;
        assert_eq!(db_url, "postgres://user:password@localhost/test");

        Ok(())
    }

    #[cfg(feature = "actix")]
    #[test]
    fn test_actix_server() -> Result<(), Box<dyn Error>> {
        let config = Config::from_str("debug=true")?;
        let server = super::actix::Server;
        server.run(&config, "")?;

        Ok(())
    }
}
