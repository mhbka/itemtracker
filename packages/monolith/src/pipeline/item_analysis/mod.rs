use crate::{config::ItemAnalysisConfig, domain::pipeline_states::{GalleryItemAnalysisState, GalleryItemEmbedderState}};
use analyzer::Analyzer;
use error::ItemAnalysisError;

pub mod error;
mod analyzer;

/// Handles analysis of items.
#[derive(Clone)]
pub struct ItemAnalyzer {
    analyzer: Analyzer
}

impl ItemAnalyzer {
    /// Initialize the struct.
    pub fn new(config: &ItemAnalysisConfig) -> Self {
        Self {
            analyzer: Analyzer::new(config)
        }
    }

    /// Perform item analysis on items.
    pub async fn analyze(&mut self, gallery_state: GalleryItemAnalysisState) -> Result<GalleryItemEmbedderState, ItemAnalysisError> {
        let analyzed_items = self.analyzer
            .analyze_gallery(gallery_state.items, &gallery_state.evaluation_criteria)
            .await;

        let new_state = GalleryItemEmbedderState {
            gallery_id: gallery_state.gallery_id,
            items: analyzed_items,
            marketplace_updated_datetimes: gallery_state.marketplace_updated_datetimes,
            failed_marketplace_reasons: gallery_state.failed_marketplace_reasons,
            used_evaluation_criteria: gallery_state.evaluation_criteria,
        };

        Ok(new_state)
    }
}