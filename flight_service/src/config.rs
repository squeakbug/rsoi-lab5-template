use std::env;
use crate::error::ConfigError;

#[derive(Debug, Clone)]
pub struct Config {
    pub listen_address: String,
    pub database_url: String,

    pub okta_oauth_client_id: String,
    pub okta_oauth_client_secret: String,
    pub okta_oauth_domain: String,
    pub okta_oauth_key: String,
}

#[allow(unused)]
fn config(name: &str) -> Result<String, ConfigError> {
    match env::var(name) {
        Err(_) => dotenv::var(name).map_err(|_| ConfigError {
            message: format!("{} must be set", name),
        }),
        Ok(res) => Ok(res),
    }
}

#[allow(unused)]
fn config_default(name: &str, default: &str) -> String {
    env::var(name).unwrap_or(dotenv::var(name).unwrap_or(default.into()))
}

impl Config {
    pub fn init() -> Result<Config, ConfigError> {
        let listen_address = config_default("LISTEN_ADDRESS", "0.0.0.0:8080");
        let database_url = config("DATABASE_URL")?;

        let okta_oauth_client_id = config("OKTA_OAUTH_CLIENT_ID")?;
        let okta_oauth_client_secret = config("OKTA_OAUTH_CLIENT_SECRET")?;
        let okta_oauth_domain = config("OKTA_OAUTH_DOMAIN")?;
        let okta_oauth_key = config("OKTA_OAUTH_KEY")?;

        let config = Config {
            listen_address,
            database_url,
            okta_oauth_client_id,
            okta_oauth_client_secret,
            okta_oauth_domain,
            okta_oauth_key,
        };

        Ok(config)
    }
}
