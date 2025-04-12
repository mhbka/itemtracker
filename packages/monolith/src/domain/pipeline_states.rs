//! This module holds types related to each stage of the scraping pipeline.
//! We can map each stage's state to the next stage using `map_to_next_state`.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::{
    domain_types::{GalleryId, ItemId, Marketplace, UnixUtcDateTime, ValidCronString}, eval_criteria::EvaluationCriteria, item_data::MarketplaceItemData, pipeline_items::{MarketplaceAnalyzedItems, MarketplaceEmbeddedAndAnalyzedItems}, search_criteria::SearchCriteria
};


/// The possible states of a gallery in the scraping pipeline.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GalleryPipelineStates {
    Initialization(GallerySchedulerState),
    SearchScraping(GallerySearchScrapingState),
    ItemScraping(GalleryItemScrapingState),
    ItemAnalysis(GalleryItemAnalysisState),
    ItemEmbedding(GalleryItemEmbedderState),
    Final(GalleryFinalState)
}

impl GalleryPipelineStates {
    /// Returns if the state type matches the state.
    pub fn matches(&self, state_type: &GalleryPipelineStateTypes) -> bool {
        matches!(
            (self, state_type),
            (GalleryPipelineStates::Initialization(_), GalleryPipelineStateTypes::Initialization) |
            (GalleryPipelineStates::SearchScraping(_), GalleryPipelineStateTypes::SearchScraping) |
            (GalleryPipelineStates::ItemScraping(_), GalleryPipelineStateTypes::ItemScraping) |
            (GalleryPipelineStates::ItemAnalysis(_), GalleryPipelineStateTypes::ItemAnalysis) |
            (GalleryPipelineStates::ItemEmbedding(_), GalleryPipelineStateTypes::ItemEmbedding) |
            (GalleryPipelineStates::Final(_), GalleryPipelineStateTypes::Final)
        )
    }

    /// Returns the state's state type.
    pub fn state_type(&self) -> GalleryPipelineStateTypes {
        match self {
            GalleryPipelineStates::Initialization(_) => GalleryPipelineStateTypes::Initialization,
            GalleryPipelineStates::SearchScraping(_) => GalleryPipelineStateTypes::SearchScraping,
            GalleryPipelineStates::ItemScraping(_) => GalleryPipelineStateTypes::ItemScraping,
            GalleryPipelineStates::ItemAnalysis(_) => GalleryPipelineStateTypes::ItemAnalysis,
            GalleryPipelineStates::ItemEmbedding(_) => GalleryPipelineStateTypes::ItemEmbedding,
            GalleryPipelineStates::Final(_) => GalleryPipelineStateTypes::Final,
        }
    }
}

/// A stateless enum of the possible states in the pipeline.
/// 
/// Used for matching on the stateful version using its `matches` function.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GalleryPipelineStateTypes {
    Initialization, 
    SearchScraping, 
    ItemScraping,
    ItemAnalysis,
    ItemEmbedding,
    Final
}

impl GalleryPipelineStateTypes {
    /// Returns if the state type matches the state.
    pub fn matches(&self, state: &GalleryPipelineStates) -> bool {
        state.matches(self)
    }
}

/// This is the state of a gallery in the scheduler.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GallerySchedulerState {
    pub gallery_id: GalleryId,
    pub scraping_periodicity: ValidCronString,
    pub search_criteria: SearchCriteria,
    pub marketplace_previous_scraped_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub evaluation_criteria: EvaluationCriteria,
}

/// This is the initial state that a scraping job starts in.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GallerySearchScrapingState {
    pub gallery_id: GalleryId,
    pub search_criteria: SearchCriteria,
    pub marketplace_previous_scraped_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub evaluation_criteria: EvaluationCriteria,
}

/// This is the state of a gallery after it has been search-scraped.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryItemScrapingState {
    pub gallery_id: GalleryId,
    pub item_ids: HashMap<Marketplace, Vec<ItemId>>,
    pub marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub failed_marketplace_reasons: HashMap<Marketplace, String>,
    pub evaluation_criteria: EvaluationCriteria,
}

/// This is the state of a scraping job after the items are item-scraped.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryItemAnalysisState {
    pub gallery_id: GalleryId,
    pub items: HashMap<Marketplace, Vec<MarketplaceItemData>>,
    pub marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub failed_marketplace_reasons: HashMap<Marketplace, String>,
    pub evaluation_criteria: EvaluationCriteria,
}

/// This is the state of a gallery after its items are analyzed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryItemEmbedderState {
    pub gallery_id: GalleryId,
    pub items: HashMap<Marketplace, MarketplaceAnalyzedItems>,
    pub marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub failed_marketplace_reasons: HashMap<Marketplace, String>,
    pub used_evaluation_criteria: EvaluationCriteria,
}

/// This is the state of a gallery after its items are embedded.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryFinalState {
    pub gallery_id: GalleryId,
    pub items: HashMap<Marketplace, MarketplaceEmbeddedAndAnalyzedItems>,
    pub marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub failed_marketplace_reasons: HashMap<Marketplace, String>,
    pub used_evaluation_criteria: EvaluationCriteria,
}