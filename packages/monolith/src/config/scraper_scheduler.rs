use std::env::VarError;

use serde::{Deserialize, Serialize};

/// Config for the scraper scheduler module.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScraperSchedulerConfig {

}

impl ScraperSchedulerConfig {
    /// Load the config from env vars. Returns a `VarError` if any are missing.
    pub(super) fn load() -> Result<Self, VarError> {
        Ok(
            ScraperSchedulerConfig {

            }
        )
    }
}