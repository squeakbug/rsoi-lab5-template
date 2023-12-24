use std::env;
use crate::error::ConfigError;

#[derive(Debug, Clone)]
pub struct Config {
    pub listen_address: String,
    pub bonus_service_address : String,
    pub flight_service_address : String,
    pub ticket_service_address : String,

    pub jwt_secret: String, 
    pub jwt_expires_in: String,
    pub jwt_max_age: i64,

    pub okta_oauth_client_id: String,
    pub okta_oauth_client_secret: String,
    pub okta_oauth_redirect_url: String,
    pub okta_oauth_domain: String,
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
        let bonus_service_address = config_default("BONUS_SERVICE_ADDRESS", "http://127.0.0.1:8050");
        let flight_service_address = config_default("FLIGHT_SERVICE_ADDRESS", "http://127.0.0.1:8060");
        let ticket_service_address = config_default("TICKET_SERVICE_ADDRESS", "http://127.0.0.1:8070");

        let jwt_secret = config("TOKEN_RSOI_SECRET")?;
        let jwt_expires_in = config("TOKEN_RSOI_EXPIRED_IN")?;
        let jwt_max_age = config("TOKEN_RSOI_MAX_AGE")?;

        let okta_oauth_client_id = config("OKTA_OAUTH_CLIENT_ID")?;
        let okta_oauth_client_secret = config("OKTA_OAUTH_CLIENT_SECRET")?;
        let okta_oauth_redirect_url = config("OKTA_OAUTH_REDIRECT_URL")?;
        let okta_oauth_domain = config("OKTA_OAUTH_DOMAIN")?;

        let config = Config {
            listen_address,
            bonus_service_address,
            flight_service_address,
            ticket_service_address,
            jwt_secret,
            jwt_expires_in,
            jwt_max_age: jwt_max_age.parse::<i64>().unwrap(),
            okta_oauth_client_id,
            okta_oauth_client_secret,
            okta_oauth_redirect_url,
            okta_oauth_domain,
        };

        Ok(config)
    }
}
