//! This module holds types related to each stage of the scraping pipeline.
//! We can map each stage's state to the next stage using `map_to_next_stage`.

use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use super::{
    domain_types::{GalleryId, Marketplace, UnixUtcDateTime, ValidCronString}, eval_criteria::EvaluationCriteria, items::pipeline_items::{
        AnalyzedItems, 
        ClassifiedItems, 
        ScrapedItems
    }, search_criteria::GallerySearchCriteria
};

/// This is the initial state that a gallery starts in.
/// 
/// Initialized in the web backend module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryInitializationState {
    pub gallery_id: GalleryId,
    pub scraping_periodicity: ValidCronString,
    pub search_criteria: GallerySearchCriteria,
    pub marketplace_previous_scraped_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub evaluation_criteria: EvaluationCriteria,
}

impl GalleryInitializationState {
    /// Convenience fn for mapping to the next state.
    pub fn to_next_stage(self) -> GalleryScrapingState {
        GalleryScrapingState {
            gallery_id: self.gallery_id,
            search_criteria: self.search_criteria,
            marketplaces_updated_datetimes: self.marketplace_previous_scraped_datetimes,
            failed_marketplace_reasons: HashMap::new(),
            evaluation_criteria: self.evaluation_criteria,
        }
    }
}

/// This is the initial state that a scraping job starts in.
/// 
/// Initialized in the scraper scheduler module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryScrapingState {
    pub gallery_id: GalleryId,
    pub search_criteria: GallerySearchCriteria,
    pub marketplaces_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
    pub failed_marketplace_reasons: HashMap<Marketplace, String>,
    pub evaluation_criteria: EvaluationCriteria,
}

impl GalleryScrapingState {

}

/// This is the state of a marketplace after it has been search-scraped.


/// This is the state of a scraping job after the items are scraped.
/// 
/// Initialized in the scraper module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryScrapedState {
    pub gallery_id: GalleryId,
    pub items: ScrapedItems,
    pub evaluation_criteria: EvaluationCriteria,
}

impl GalleryScrapedState {

}

/// This is the state of a scraping State after its items are analyzed.
/// 
/// Initialized in the item analysis module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryAnalyzedState {
    pub gallery_id: GalleryId,
    pub items: AnalyzedItems,
    pub evaluation_criteria: EvaluationCriteria // TODO: do I still need this here?
}

impl GalleryAnalyzedState {

}

/// This is the state of a scraping State after its items are classified into groups within the gallery.
/// 
/// Initialized in the image classifier module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryClassifiedState {
    pub gallery_id: GalleryId,
    pub items: ClassifiedItems,
}

impl GalleryClassifiedState {

}