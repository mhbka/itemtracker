use serde::{Serialize, Deserialize};
use crate::galleries::eval_criteria::{CriterionAnswer, EvaluationCriteria};
use super::item_data::MarketplaceItemData;

/* 
/// Items that have been classified in the image classifier module.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClassifiedItems {
    pub items: HashMap<Marketplace, Vec<ClassifiedMarketplaceItem>>,
    pub error_items: HashMap<Marketplace, Vec<MarketplaceItemData>> 
}
*/

/// All analyzed items under a marketplace.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarketplaceAnalyzedItems {
    pub relevant_items: Vec<AnalyzedMarketplaceItem>,
    pub irrelevant_items: Vec<AnalyzedMarketplaceItem>,
    pub error_items: Vec<ErrorAnalyzedMarketplaceItem>
}

/// All embedded items under a marketplace, as well as irrelevant/error analyzed items.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MarketplaceEmbeddedAndAnalyzedItems {
    pub embedded_items: Vec<EmbeddedMarketplaceItem>,
    pub irrelevant_analyzed_items: Vec<AnalyzedMarketplaceItem>,
    pub error_analyzed_items: Vec<ErrorAnalyzedMarketplaceItem>,
    pub error_embedded_items: Vec<ErrorEmbeddedMarketplaceItem>
}

/// An item under a marketplace, whose description and image has been embedded.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmbeddedMarketplaceItem {
    pub item: MarketplaceItemData,
    pub evaluation_answers: Vec<CriterionAnswer>,
    pub item_description: String,
    pub description_embedding: Vec<f32>,
    pub image_embedding: Vec<f32>
}

/// An analyzed item under a marketplace.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnalyzedMarketplaceItem {
    pub item: MarketplaceItemData,
    pub evaluation_answers: Vec<CriterionAnswer>,
    pub item_description: String,
    pub best_fit_image: usize
}

/// An item which encountered an error during analysis.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorAnalyzedMarketplaceItem {
    pub item: MarketplaceItemData,
    pub error: String
}

/// An item which encountered an error during embedding.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorEmbeddedMarketplaceItem {
    pub item: AnalyzedMarketplaceItem,
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
