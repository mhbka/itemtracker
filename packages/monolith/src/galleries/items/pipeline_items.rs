use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::galleries::{domain_types::Marketplace, eval_criteria::{CriterionAnswer, EvaluationCriteria}};
use super::item_data::MarketplaceItemData;

/// Items that have been classified in the image classifier module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassifiedItems {
    pub items: HashMap<Marketplace, Vec<ClassifiedMarketplaceItem>>,
    pub error_items: HashMap<Marketplace, Vec<MarketplaceItemData>> 
}

/// All analyzed items under a marketplace.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarketplaceAnalyzedItems {
    pub relevant_items: Vec<AnalyzedMarketplaceItem>,
    pub irrelevant_items: Vec<AnalyzedMarketplaceItem>,
    pub error_items: Vec<ErrorAnalyzedMarketplaceItem>
}

/// An analyzed item under a marketplace.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzedMarketplaceItem {
    pub item: MarketplaceItemData,
    pub evaluation_answers: Vec<CriterionAnswer>
}

/// Analyzed items under a marketplace.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorAnalyzedMarketplaceItem {
    pub item: MarketplaceItemData,
    pub error: String
}

/// Classified items under a marketplace.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassifiedMarketplaceItem {
    pub item: MarketplaceItemData,
    pub answers: EvaluationCriteria,
    pub gallery_group_id: String,
    pub is_new_group: bool
}
