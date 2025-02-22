use std::env::{self, VarError};

use serde::{Deserialize, Serialize};

/// Config for the image classifier module.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemEmbedderConfig {
    pub embedder_endpoint: String
}

impl ItemEmbedderConfig {
    /// Load the config from env vars. Returns a `VarError` if any are missing.
    pub(super) fn load() -> Result<Self, VarError> {
        Ok(
            ItemEmbedderConfig {
                embedder_endpoint: env::var("EMBEDDER_ENDPOINT")?,
            }
        )
    }
}