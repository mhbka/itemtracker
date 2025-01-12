use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::galleries::{eval_criteria::EvaluationCriteria, domain_types::Marketplace};
use super::item_data::MarketplaceItemData;

/// Items that have been freshly scraped in the scraper module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScrapedItems {
    pub marketplace_items: HashMap<Marketplace, Vec<MarketplaceItemData>>
}

/// Items that have been analyzed in the item analysis module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzedItems {
    pub items: HashMap<Marketplace, Vec<AnalyzedMarketplaceItem>>,
    pub error_items: HashMap<Marketplace, Vec<MarketplaceItemData>>
}

/// Items that have been classified in the image classifier module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassifiedItems {
    pub items: HashMap<Marketplace, Vec<ClassifiedMarketplaceItem>>,
    pub error_items: HashMap<Marketplace, Vec<MarketplaceItemData>> 
}

///// Subtypes

/// Analyzed items under a marketplace.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzedMarketplaceItem {
    pub is_relevant: bool,
    pub item: MarketplaceItemData,
    pub evaluated_criteria: EvaluationCriteria
}

/// Classified items under a marketplace.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassifiedMarketplaceItem {
    pub item: MarketplaceItemData,
    pub answers: EvaluationCriteria,
    pub gallery_group_id: String,
    pub is_new_group: bool
}
