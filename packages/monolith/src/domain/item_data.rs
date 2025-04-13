use serde::{Deserialize, Serialize};
use crate::domain::domain_types::{ItemId, UnixUtcDateTime};

/// This is the data for each item, common across all marketplaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceItemData {
    pub item_id: ItemId,
    pub name: String,
    pub price: f64,  
    pub description: String,
    pub status: String,
    pub seller_id: String,
    pub category: String,
    pub thumbnails: Vec<String>, 
    pub item_condition: String,
    pub created: UnixUtcDateTime,
    pub updated: UnixUtcDateTime,
}