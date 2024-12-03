use std::collections::HashMap;
use chrono::{DateTime, Utc};

use crate::{galleries::
    {
        domain_types::{GalleryId, ItemId}, 
        items::item_data::MarketplaceItemData, domain_types::Marketplace
    }, 
        messages::{message_types::storage::marketplace_items::{FetchItems, FetchItemsMessage, MarketplaceItemsStorageMessage}, MarketplaceItemsStorageSender}
    };

/// This functions as the interface for fetching stored marketplace items, storing them locally (temporarily).
/// 
/// The flow is to fetch some items from storage using their item IDs, keeping it in the item cache.
/// Items that could not be fetched (ie were never scraped before) will be scraped.
/// 
/// Then, the fetched and newly scraped items are combined and sent to the next stage.
pub struct ItemCache {
    item_storage_msg_sender: MarketplaceItemsStorageSender,
    cached_items: HashMap<(GalleryId, Marketplace), Vec<MarketplaceItemData>>
}

impl ItemCache {
    /// Initialize the item cache.
    pub(super) fn new(item_storage_msg_sender: MarketplaceItemsStorageSender) -> Self {
        Self { 
            item_storage_msg_sender,
            cached_items: HashMap::new()
        }
    }

    /// Fetch any stored marketplace items for the given item IDs under the given marketplace, up to a given earlier datetime, 
    /// and cache it under the given gallery ID and marketplace.
    /// 
    /// Returns a Vec of item IDs that were not found in storage (ie need to be scraped).
    pub(super) async fn fetch_from_storage(
        &mut self, 
        gallery_id: GalleryId,
        up_to: DateTime<Utc>,
        marketplace: Marketplace, 
        item_ids: Vec<ItemId>,
    ) -> Vec<ItemId> 
    {   
        // If this gallery already has cached items under this marketplace, check against those
        if self.cached_items.contains_key(&(gallery_id.clone(), marketplace.clone())) {
            return self.filter_from_cache(gallery_id, marketplace, item_ids);
        }

        // Else, fetch from storage
        let raw_msg = FetchItems { marketplace: marketplace.clone(), item_ids, up_to };
        let (msg, response_receiver) = FetchItemsMessage::new(raw_msg);
        self.item_storage_msg_sender
            .send(MarketplaceItemsStorageMessage::FetchItems(msg))
            .await;

        match response_receiver.await {
            Ok(res) => {
                self.cached_items.insert(
                    (gallery_id.clone(), marketplace.clone()),
                    res.stored_items
                );
                res.unfetched_marketplace_item_ids
            },
            Err(_) => {
                // TODO: handle this error somehow
                Vec::new()
            },
        }
    }

    /// Obtain previously-fetched-from-storage items for this gallery ID and marketplace,
    /// and remove them from the cache. 
    pub(super) fn get_from_cache(
        &mut self, 
        marketplace: Marketplace, 
        gallery_id: GalleryId
    ) -> Vec<MarketplaceItemData> {
        self.cached_items
            .remove(&(gallery_id, marketplace))
            .unwrap_or(Vec::new())
    }

    /// If items under a given gallery and marketplace are already cached,
    /// use this function to filter out which item IDs still need to be scraped.
    fn filter_from_cache(
        &self, 
        gallery_id: GalleryId,
        marketplace: Marketplace, 
        item_ids: Vec<ItemId>,
    ) -> Vec<ItemId> {
        let new_gallery_items = Vec::new();
        let mut cached_gallery_items = self.cached_items
            .get(&(gallery_id, marketplace))
            .unwrap_or(&new_gallery_items)
            .into_iter();
        
        item_ids
            .into_iter()
            .filter(|id| cached_gallery_items.find(|&item| item.id == *id).is_none())
            .collect()
    }
}