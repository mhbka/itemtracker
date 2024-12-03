use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::galleries::domain_types::{ItemId, UnixUtcDateTime};

/// This is the data for each item, common across all marketplaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceItemData {
    pub id: ItemId,
    pub name: String,
    pub price: f32,  
    pub description: String,
    pub status: ItemStatus,
    pub seller: Seller,
    pub category: String,
    pub image_urls: Vec<String>, 
    pub item_condition: ItemCondition,
    pub created: UnixUtcDateTime,
    pub updated: UnixUtcDateTime,
}

/// Data for the item's seller.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seller {
    pub id: String,
    pub name: String,
    pub ratings: f32,
}

/// Possible item statuses.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemStatus {
    Available,
    Sold,
    Reserved,
    Deleted,
}

/// Possible item conditions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemCondition {
    New,
    LikeNew,
    Good,
    Fair,
    Poor,
}