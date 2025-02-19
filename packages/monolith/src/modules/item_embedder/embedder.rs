use crate::config::ItemEmbedderConfig;

/// In charge of handling requests to the actual embedding service.
pub(super) struct Embedder {
    config: ItemEmbedderConfig
}

impl Embedder {
    pub fn new(config: ItemEmbedderConfig) -> Self {
        Self {
            config
        }
    }
}