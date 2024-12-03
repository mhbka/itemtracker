use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::{galleries::{domain_types::{Marketplace, ItemId}, items::item_data::MarketplaceItemData}, messages::message_types::ModuleMessageWithReturn};

/// Types of messages the marketplace items storage module can take.
#[derive(Debug)]
pub enum MarketplaceItemsStorageMessage {
    FetchItems(FetchItemsMessage),
    StoreItems(StoreItemsMessage)
}

/// Message to fetch cached marketplace items.
pub type FetchItemsMessage = ModuleMessageWithReturn<FetchItems, FetchItemsResponse>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FetchItems {
    pub marketplace: Marketplace,
    pub item_ids: Vec<ItemId>,
    pub up_to: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FetchItemsResponse {
    pub stored_items: Vec<MarketplaceItemData>,
    pub unfetched_marketplace_item_ids: Vec<ItemId>
}

/// Message to store marketplace items.
pub type StoreItemsMessage = ModuleMessageWithReturn<StoreItems, ()>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StoreItems {
    pub marketplace: Marketplace,
    pub items: Vec<MarketplaceItemData>
}

