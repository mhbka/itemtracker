use std::env::VarError;

use serde::{Deserialize, Serialize};

/// Config for the image analysis module.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageAnalysisConfig {

}

impl ImageAnalysisConfig {
    /// Load the config from env vars. Returns a `VarError` if any are missing.
    pub(super) fn load() -> Result<Self, VarError> {
        Ok(
            ImageAnalysisConfig {

            }
        )
    }
}