//! This module holds types related to each stage of the scraping pipeline.
//! We can map each stage's state to the next stage using `map_to_next_stage`.

use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use super::{
    domain_types::{Marketplace, GalleryId, ValidCronString}, eval_criteria::EvaluationCriteria, items::pipeline_items::{
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
    pub marketplaces: HashSet<Marketplace>,
    pub previous_scraped_item_datetime: DateTime<Utc>,
    pub evaluation_criteria: EvaluationCriteria,
}

impl GalleryInitializationState {
    /// Map this state to a `GalleryScrapingState`.
    pub fn map_to_next_stage(self) -> GalleryScrapingState {
        GalleryScrapingState {
            gallery_id: self.gallery_id,
            search_criteria: self.search_criteria,
            marketplaces: self.marketplaces,
            previous_scraped_item_datetime: self.previous_scraped_item_datetime,
            evaluation_criteria: self.evaluation_criteria
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
    pub marketplaces: HashSet<Marketplace>,
    pub previous_scraped_item_datetime: DateTime<Utc>,
    pub evaluation_criteria: EvaluationCriteria,
}

impl GalleryScrapingState {
    pub fn map_to_next_stage(self, items: ScrapedItems) -> GalleryScrapedState {
        GalleryScrapedState {
            gallery_id: self.gallery_id,
            items,
            evaluation_criteria: self.evaluation_criteria
        }
    }
}

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
    pub fn map_to_next_stage(self, items: AnalyzedItems) -> GalleryAnalyzedState {
        GalleryAnalyzedState {
            gallery_id: self.gallery_id,
            items
        }
    }
}

/// This is the state of a scraping State after its items are analysed.
/// 
/// Initialized in the image analysis module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryAnalyzedState {
    pub gallery_id: GalleryId,
    pub items: AnalyzedItems,
}

impl GalleryAnalyzedState {
    pub fn map_to_next_stage(self, items: ClassifiedItems) -> GalleryClassifiedState {
        GalleryClassifiedState {
            gallery_id: self.gallery_id,
            items
        }
    }
}


/// This is the state of a scraping State after its items are classified into groups within the gallery.
/// 
/// Initialized in the image classifier module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GalleryClassifiedState {
    pub gallery_id: GalleryId,
    pub items: ClassifiedItems,
}