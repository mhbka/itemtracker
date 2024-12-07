use std::{collections::{HashMap, HashSet}, sync::Arc};
use tokio::sync::Mutex;
use tracing::Instrument;
use crate::{galleries::{domain_types::{GalleryId, ItemId, Marketplace, UnixUtcDateTime}, eval_criteria::{self, EvaluationCriteria}, items::{item_data::MarketplaceItemData, pipeline_items::ScrapedItems}, scraping_pipeline::GalleryScrapedState}, messages::{message_types::{img_analysis::{ImgAnalysisMessage, StartAnalysisJob, StartAnalysisJobMessage}, scraper::ScraperError, storage::marketplace_items::{FetchItems, FetchItemsMessage, MarketplaceItemsStorageMessage}}, ImgAnalysisSender, MarketplaceItemsStorageSender}};

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
    img_analysis_msg_sender: ImgAnalysisSender
}

impl OutputProcessor {
    /// Instantiate the `OutputProcessor`.
    pub fn new(
        item_storage_msg_sender: MarketplaceItemsStorageSender,
        img_analysis_msg_sender: ImgAnalysisSender
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
        self.item_storage_msg_sender
            .send(MarketplaceItemsStorageMessage::FetchItems(msg))
            .await
            .unwrap(); // TODO: proper error handling here
        match response_receiver.await {
            Ok(res) => {
                self.item_storage.insert(storage_key, res.stored_items);
                return res.unfetched_marketplace_item_ids;
            },
            Err(_) => {
                // If we have RecvError, just log it and return all the item IDs
                tracing::error!("RecvError while fetching cached items from storage; will scrape all item IDs..."); // TODO: better error message
                self.item_storage.insert(storage_key, vec![]);
                return scraped_item_ids;
            },
        }
    }

    /// Processes newly scraped items from a marketplace and temporarily stores them until they are to be sent.
    pub async fn process_scraped_items(
        &mut self,
        gallery_id: GalleryId,
        marketplace: Marketplace,
        mut scraped_items: Vec<MarketplaceItemData>
    ) { 
        let storage_key = (gallery_id, marketplace);
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
    }

    /// Sends all items under a gallery to the next stage.
    pub async fn send_gallery_items(
        &mut self,
        gallery_id: GalleryId,
        eval_criteria: EvaluationCriteria
    ) -> Result<(), ScraperError>
    {
        // TODO: this is pretty ugly, surely there's a way to just filter them without clone
        let mut marketplace_items = HashMap::new();
        self.item_storage.retain(|(stored_gallery_id, marketplace), items| {
            if *stored_gallery_id == gallery_id {
                marketplace_items.insert(marketplace.clone(), items.clone());
                return false;
            }
            true
        });
        let job_msg = StartAnalysisJob::build(
            gallery_id, 
            eval_criteria, 
            marketplace_items
        );
        let (msg, response_receiver) = StartAnalysisJobMessage::new(job_msg);
        self.img_analysis_msg_sender
            .send(ImgAnalysisMessage::StartAnalysis(msg))
            .await
            .unwrap(); // TODO: proper error handling here
        if response_receiver.await.is_err() {
            tracing::warn!("RecvError while receiving response when sending gallery items");
        }
        Ok(())
    }
}