use serde::{Deserialize, Serialize};
use crate::galleries::domain_types::{ItemId, UnixUtcDateTime};

/// This is the data for each item, common across all marketplaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceItemData {
    pub id: ItemId,
    pub name: String,
    pub price: f32,  
    pub description: String,
    pub status: String,
    pub seller: MarketplaceSeller,
    pub category: String,
    pub thumbnails: Vec<String>, 
    pub item_condition: String,
    pub created: UnixUtcDateTime,
    pub updated: UnixUtcDateTime,
}

/// Data for the item's seller.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceSeller {
    pub id: String, 
    pub name: String,
}