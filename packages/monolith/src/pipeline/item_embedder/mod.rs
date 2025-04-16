use std::collections::HashMap;
use crate::{
    config::ItemEmbedderConfig, 
    domain::{domain_types::{GalleryId, Marketplace, UnixUtcDateTime}, eval_criteria::EvaluationCriteria, pipeline_items::MarketplaceEmbeddedAndAnalyzedItems, pipeline_states::{GalleryFinalState, GalleryItemEmbedderState, GalleryPipelineStateTypes, GalleryPipelineStates}}, 
};
use embedder::Embedder;
use error::ItemEmbedderError;

mod embedder;
pub mod error;

/// Handles embedding of items in the pipeline.
#[derive(Clone)]
pub struct ItemEmbedder {
    embedder: Embedder
}

impl ItemEmbedder {
    /// Instantiate the module.
    pub fn new(config: &ItemEmbedderConfig) -> Self {
        Self {
            embedder: Embedder::new(config)
        }
    }

    /// Embed the items in the gallery session.
    pub async fn embed(&mut self, gallery_state: GalleryItemEmbedderState) -> Result<GalleryFinalState, ItemEmbedderError> {
        let embedded_items = self.embedder
            .embed_gallery(gallery_state.items)
            .await;

        let next_state = GalleryFinalState {
            gallery_id: gallery_state.gallery_id,
            items: embedded_items,
            marketplace_updated_datetimes: gallery_state.marketplace_updated_datetimes,
            failed_marketplace_reasons: gallery_state.failed_marketplace_reasons,
            used_evaluation_criteria: gallery_state.used_evaluation_criteria,
        };

        Ok(next_state)
    }
}