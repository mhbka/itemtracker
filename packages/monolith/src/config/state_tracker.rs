use std::env::{self, VarError};

use serde::{Deserialize, Serialize};

/// Config for the scraper module.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StateTrackerConfig {
    pub use_redis: bool,
    pub redis_uri: String
}

impl StateTrackerConfig {
    /// Load the config from env vars. Returns a `VarError` if any are missing.
    pub(super) fn load() -> Result<Self, VarError> {
        let use_redis = match env::var("USE_REDIS")?.as_str() {
            "true" => true,
            "false" => false,
            _ => false
        };
        Ok(
            Self {
                use_redis,
                redis_uri: env::var("REDIS_URI")?
            }
        )
    }
}