use std::env::VarError;

use serde::{Deserialize, Serialize};

/// Config for the image classifier module.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageClassifierConfig {

}

impl ImageClassifierConfig {
    /// Load the config from env vars. Returns a `VarError` if any are missing.
    pub(super) fn load() -> Result<Self, VarError> {
        Ok(
            ImageClassifierConfig {

            }
        )
    }
}