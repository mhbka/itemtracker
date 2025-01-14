use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::galleries::{domain_types::{GalleryId, Marketplace}, eval_criteria::EvaluationCriteria, items::{item_data::MarketplaceItemData, pipeline_items::ScrapedItems}, pipeline_states::GalleryScrapedState};

/// Possible errors emitted from the item analysis module.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum ItemAnalysisError {
}

/// The types of messages the item analysis module can take.
#[derive(Debug)]
pub enum ItemAnalysisMessage {
    StartAnalysis(StartAnalysisJob)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StartAnalysisJob {
    pub gallery: GalleryScrapedState
}

impl StartAnalysisJob {
    /// Convenience function for building this struct.
    pub fn build(
        gallery_id: GalleryId,
        eval_criteria: EvaluationCriteria,
        marketplace_items: HashMap<Marketplace, Vec<MarketplaceItemData>>
    ) -> Self 
    {
        let scraped_items = ScrapedItems { marketplace_items };
        let scraped_gallery = GalleryScrapedState { 
            gallery_id, 
            items: scraped_items, 
            evaluation_criteria: eval_criteria
        };
        StartAnalysisJob { gallery: scraped_gallery }
    }
}