use std::collections::HashMap;
use crate::{
    galleries::{domain_types::{GalleryId, ItemId, Marketplace, UnixUtcDateTime}, 
    eval_criteria::{self, EvaluationCriteria}, items::{item_data::MarketplaceItemData, pipeline_items::ScrapedItems}, 
    pipeline_states::GalleryScrapedState}, messages::{message_types::{item_analysis::ItemAnalysisMessage, scraper::ScraperError, storage::marketplace_items::{FetchItems, FetchItemsMessage, MarketplaceItemsStorageMessage}}, ItemAnalysisSender, MarketplaceItemsStorageSender}};

/// This fetches cached items, processes scraped items, and eventually sends them to the next stage. 
/// 
/// Its functionality is (currently) very simple:
/// - Call `fetch_cached_items` to fetch items that were previously scraped, and get the item IDs that still need to be scraped
/// - Call `process_scraped_items` to combine newly scraped items + fetched cached items, and temporarily store it
/// - Call `send_items` to combine all scraped marketplace data under a gallery, and send it through the `img_analysis_msg_sender`
/// 
/// **NOTE**: It is up to the caller to ensure that all marketplaces for a gallery have been fully scraped before sending them.
pub(super) struct OutputProcessor {
    item_storage: HashMap<(GalleryId, Marketplace), Vec<MarketplaceItemData>>,
    item_storage_msg_sender: MarketplaceItemsStorageSender,
    img_analysis_msg_sender: ItemAnalysisSender
}

impl OutputProcessor {
    /// Instantiate the `OutputProcessor`.
    pub fn new(
        item_storage_msg_sender: MarketplaceItemsStorageSender,
        img_analysis_msg_sender: ItemAnalysisSender
    ) -> Self {
        Self { 
            item_storage: HashMap::new(),
            item_storage_msg_sender,
            img_analysis_msg_sender 
        }
    }

    /// Fetches item IDs in `scraped_item_ids` which have been scraped before and stores them locally,
    /// and returns item IDs which have never been scraped before (and must be scraped).
    pub async fn fetch_cached_items(
        &mut self,
        gallery_id: &GalleryId,
        marketplace: &Marketplace,
        up_to: &UnixUtcDateTime,
        scraped_item_ids: Vec<ItemId>,
    ) -> Vec<ItemId> {
        tracing::trace!("Attempting to fetch cached items for gallery {gallery_id} ({marketplace})");
        let storage_key = (gallery_id.clone(), marketplace.clone());
        if self.item_storage.contains_key(&storage_key) {
            tracing::warn!("Tried to fetch cached items for gallery {gallery_id} {marketplace} when they were already fetched");
            return scraped_item_ids;
        }
        let msg_data = FetchItems { 
            marketplace: marketplace.clone(), 
            item_ids: scraped_item_ids.clone(), 
            up_to: up_to.clone()
        };
        let (msg, response_receiver) = FetchItemsMessage::new(msg_data);
        if let Err(err) = self.item_storage_msg_sender
            .send(MarketplaceItemsStorageMessage::FetchItems(msg))
            .await {
                tracing::error!("Error while sending message to fetch items from item storage (will scrape all item IDs): {err}");
                self.item_storage.insert(storage_key, vec![]);
                return scraped_item_ids;
            }
        match response_receiver.await {
            Ok(res) => {
                tracing::info!(
                    "Successfully fetched {} cached items for gallery {} ({})",
                    res.stored_items.len(),
                    gallery_id,
                    marketplace
                );
                self.item_storage.insert(storage_key, res.stored_items);
                return res.unfetched_marketplace_item_ids;
            },
            Err(err) => {
                // If we couldn't receive a response, just log it and return all the item IDs
                tracing::error!("Error while fetching cached items from storage: {err}"); 
                self.item_storage.insert(storage_key, vec![]);
                return scraped_item_ids;
            },
        }
    }

    /// Processes newly scraped items from a marketplace and temporarily stores them until they are to be sent.
    pub async fn process_scraped_items(
        &mut self,
        gallery_id: &GalleryId,
        marketplace: &Marketplace,
        mut scraped_items: Vec<MarketplaceItemData>
    ) { 
        tracing::trace!("Attempting to process scraped items for gallery {gallery_id} ({marketplace})");
        let storage_key = (gallery_id.clone(), marketplace.clone());
        match self.item_storage.get_mut(&storage_key) {
            Some(cached_items) => {
                cached_items.append(&mut scraped_items);
            },
            None => {
                tracing::error!("
                    Gallery {}, marketplace {} was not found in item storage (did you call this before fetching scraped items?)", 
                    storage_key.0, storage_key.1
                );
                self.item_storage.insert(storage_key, scraped_items);
            }
        };
        tracing::info!("Successfully processed scraped items for gallery {gallery_id} ({marketplace})");
    }

    /// Sends all items under a gallery to the next stage.
    pub async fn send_gallery_items(
        &mut self,
        gallery_id: GalleryId,
        eval_criteria: EvaluationCriteria
    ) -> Result<(), ScraperError>
    {   
        tracing::trace!("Attempting to send gallery items for gallery {gallery_id}");
        // TODO: use a crate `drain_filter`, or replace when it finally makes it into stable
        let mut marketplace_items = HashMap::new();
        self.item_storage.retain(|(stored_gallery_id, marketplace), items| {
            if *stored_gallery_id == gallery_id {
                marketplace_items.insert(marketplace.clone(), items.clone());
                return false;
            }
            true
        });
        let gallery = GalleryScrapedState {
            gallery_id: gallery_id.clone(),
            items: ScrapedItems { marketplace_items },
            evaluation_criteria: eval_criteria
        };
        if let Err(err) = self.img_analysis_msg_sender
            .send(ItemAnalysisMessage::StartAnalysis { gallery })
            .await {
                tracing::error!("Error while sending gallery to item analysis module: {err}");
            }
        tracing::info!("Successfully sent gallery items for gallery {gallery_id}");
        Ok(())
    }
}