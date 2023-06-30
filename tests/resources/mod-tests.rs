#[cfg(test)]

mod tests {

    use super::*;
    use std::env;

    #[test]
    fn test_cli() {
        env::set_var("ARGS", "-p 8080 -d example.com".to_string());
        let args = cli::Cli::new().get_matches();
        assert_eq!(args.value_of("port"), Some("8080"));
    }

    #[test]
    fn test_config() {
        let config = configure::Config::new().unwrap();
        assert_eq!(config.db_url, db_conn_string());
    }

    #[actix_rt::test]
    async fn test_actix_server() {
        let srv = actix_server::server().await;
        let response = srv.get("/healthz").send().await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
