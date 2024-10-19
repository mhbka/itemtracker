use std::env;

/// Holds configuration for the app, which are obtained from env vars:
/// - host_addr: The service address including its port; variable **HOST_ADDR**
/// - scraper_addr: The scraper's address including its port; variable **SCRAPER_ADDR**
/// - scraper_username: The scraper's username; variable **SCRAPER_USERNAME**
/// - scraper_password: The scraper's password; variable **SCRAPER_PASSWORD**
#[derive(Clone)]
pub struct AppConfig {
    pub host_addr: String,
    pub scraper_addr: String,
    pub scraper_username: String,
    pub scraper_password: String
}

impl AppConfig {
    /// Initializes the config from env vars.
    /// 
    /// Panics if any of the env vars are not set.
    pub fn init() -> Self {
        AppConfig {
            host_addr: env::var("HOST_ADDR").unwrap(),
            scraper_addr: env::var("SCRAPER_ADDR").unwrap(),
            scraper_username: env::var("SCRAPER_USERNAME").unwrap(),
            scraper_password: env::var("SCRAPER_PASSWORD").unwrap(),
        }
    }
}
