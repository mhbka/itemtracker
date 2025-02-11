use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::galleries::{domain_types::{ItemId, Marketplace, UnixUtcDateTime}, eval_criteria::EvaluationCriteria, items::item_data::MarketplaceItemData};

/// The states of a gallery that are being tracked,
/// along with any important ephemeral/temporary data.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GalleryState {
    /// The gallery is being search-scraped.
    SearchScraping { 
        scraped_item_ids: HashMap<Marketplace, Vec<ItemId>>,
        updated_up_to: HashMap<Marketplace, UnixUtcDateTime>,
        failed_marketplace_reasons: HashMap<Marketplace, String>,
        eval_criteria: EvaluationCriteria
    },
    /// All marketplaces for the gallery are search-scraped, and it is now being item-scraped.
    ItemScraping {
        items: HashMap<Marketplace, Vec<MarketplaceItemData>>,
        updated_up_to: HashMap<Marketplace, UnixUtcDateTime>,
        failed_marketplace_reasons: HashMap<Marketplace, String>, 
        eval_criteria: EvaluationCriteria
    },
    /// The gallery is fully scraped and its items are being analyzed.
    ItemAnalysis {
        
    },
    /// The gallery's items are analyzed and it is now being classified.
    ImageClassification {

    }
}