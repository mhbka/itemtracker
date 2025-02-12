use std::env::VarError;
use serde::{Deserialize, Serialize};

/// Config for the scraper scheduler module.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemScraperConfig {

}

impl ItemScraperConfig {
    /// Load the config from env vars. Returns a `VarError` if any are missing.
    pub(super) fn load() -> Result<Self, VarError> {
        Ok(
            Self {

            }
        )
    }
}