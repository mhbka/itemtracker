use std::{collections::{HashMap, HashSet}, sync::Arc};
use tokio::sync::Mutex;
use tracing::Instrument;
use crate::{galleries::{domain_types::{GalleryId, Marketplace}, eval_criteria::{self, EvaluationCriteria}, items::{item_data::MarketplaceItemData, pipeline_items::ScrapedItems}, scraping_pipeline::GalleryScrapedState}, messages::{message_types::{img_analysis::{ImgAnalysisMessage, StartAnalysisJob, StartAnalysisJobMessage}, scraper::ScraperError}, ImgAnalysisSender}};
use super::item_cache::ItemCache;

/// This processes scraped items and prepares them for sending to the next stage.
/// 
/// The process is quite simple. When newly scraped items arrive, it combines them with
/// cached items that were fetched after the search scrape and stores them temporarily.
/// 
/// When all marketplaces for the gallery have been scraped, another function within the 
/// `OutputProcessor` can be called to send all the items for a gallery to the next stage.
pub struct OutputProcessor {
    marketplace_items: HashMap<(GalleryId, Marketplace), Vec<MarketplaceItemData>>,
    item_cache: Arc<Mutex<ItemCache>>,
    img_analysis_msg_sender: ImgAnalysisSender
}

impl OutputProcessor {
    /// Instantiate the `OutputProcessor`.
    pub(super) fn new(
        item_cache: Arc<Mutex<ItemCache>>, 
        img_analysis_msg_sender: ImgAnalysisSender
    ) -> Self 
    {
        Self { 
            marketplace_items: HashMap::new(),
            item_cache, 
            img_analysis_msg_sender 
        }
    }

    /// Returns all marketplaces' items under a gallery that are currently temporarily stored (ie, these marketplaces have been scraped).
    pub(super) fn get_scraped_marketplaces(&self, gallery_id: GalleryId) -> HashSet<&Marketplace> {
        self.marketplace_items
            .iter()
            .filter(|&((stored_gallery_id, _), _)| *stored_gallery_id == gallery_id)
            .map(|((_, marketplace), _)| marketplace)
            .collect()
    }

    /// Processes newly scraped items from a marketplace and temporarily stores them until they are to be sent.
    pub(super) async fn process_scraped_items(
        &mut self,
        gallery_id: GalleryId,
        marketplace: Marketplace,
        mut scraped_items: Vec<MarketplaceItemData>
    ) 
    {       
        let cached_items = self.item_cache
            .lock()
            .await
            .get_from_cache(&marketplace, &gallery_id);
        scraped_items
            .extend_from_slice(&cached_items);
        self.marketplace_items.insert(
            (gallery_id, marketplace),
            scraped_items
        );
    }

    /// Sends all items under a gallery to the next stage.
    /// 
    /// Ideally this should only be called once all marketplaces for a gallery have been scraped,
    /// but you can really call it whenever you want.
    /// 
    /// Returns an `Err` if the sending fails, or the given gallery ID is not found in
    /// the `OutputProcessor`'s temporary storage.
    pub(super) async fn send_gallery_items(
        &mut self,
        gallery_id: GalleryId,
        eval_criteria: EvaluationCriteria
    ) -> Result<(), ScraperError>
    {
        // TODO: this is pretty ugly, surely there's a way to just filter them without clone
        let mut marketplace_items = HashMap::new();
        self.marketplace_items.retain(|(stored_gallery_id, marketplace), items| {
            if *stored_gallery_id == gallery_id {
                marketplace_items.insert(marketplace.clone(), items.clone());
                return false;
            }
            true
        });

        let scraped_items = ScrapedItems { marketplace_items };
        let scraped_gallery = GalleryScrapedState { 
            gallery_id, 
            items: scraped_items, 
            evaluation_criteria: eval_criteria
        };
        let inner_msg = StartAnalysisJob { gallery: scraped_gallery };
        let (msg, response_receiver) = StartAnalysisJobMessage::new(inner_msg);
        self.img_analysis_msg_sender
            .send(ImgAnalysisMessage::StartAnalysis(msg))
            .await
            .unwrap();
            // TODO: proper error handling here

        Ok(())
    }
}