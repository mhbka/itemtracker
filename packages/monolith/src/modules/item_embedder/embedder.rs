use std::collections::HashMap;

use crate::{config::ItemEmbedderConfig, galleries::{domain_types::Marketplace, items::pipeline_items::{MarketplaceAnalyzedItems, MarketplaceEmbeddedAndAnalyzedItems}, pipeline_states::GalleryItemEmbedderState}};

/// In charge of handling requests to the actual embedding service.
pub(super) struct Embedder {
    config: ItemEmbedderConfig
}

impl Embedder {
    /// Initialize the struct.
    pub fn new(config: ItemEmbedderConfig) -> Self {
        Self {
            config
        }
    }

    /// Embed a gallery's items' description and chosen images.
    pub async fn embed_gallery(&mut self, items: HashMap<Marketplace, MarketplaceAnalyzedItems>) -> HashMap<Marketplace, MarketplaceEmbeddedAndAnalyzedItems> {
        let mut embedded_items = HashMap::new();
        for (marketplace, items) in items {

        }
        embedded_items
    }
}