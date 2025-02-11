use std::env::{self, VarError};

use serde::{Deserialize, Serialize};

/// Config for the scraper module.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StateTrackerConfig {
    
}

impl StateTrackerConfig {
    /// Load the config from env vars. Returns a `VarError` if any are missing.
    pub(super) fn load() -> Result<Self, VarError> {
        Ok(
            Self {
                
            }
        )
    }
}