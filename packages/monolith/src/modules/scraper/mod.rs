use std::{collections::HashSet, sync::Arc};

use individual_items_scraper::IndividualItemsScraper;
use item_cache::ItemCache;
use output_processor::OutputProcessor;
use search_scraper::SearchScraper;
use tokio::sync::Mutex;

use crate::{config::ScraperConfig, galleries::{domain_types::GalleryId, scraping_pipeline::GalleryScrapingState}, messages::{
    message_types::scraper::ScraperMessage, ImgAnalysisSender, MarketplaceItemsStorageSender, ScraperReceiver
}};

mod individual_items_scraper;
mod search_scraper;
mod item_cache;
mod output_processor;

/// Module in charge of orchestrating actual scraping through Scrapy spiders.
/// 
/// The flow of this module is rather complex. Scraping begins from a `StartScraping` message.
/// If a new gallery is not currently being scraped, they are scheduled to scrape marketplace searches using the gallery's search criteria.
/// 
/// This scrape returns a list of item listing IDs through a HTTP endpoint exposed on the app, which
/// feeds back into this module as a `ScrapeIndividualItems` message.
/// 
/// These item IDs are sent to the item storage module through the `ItemCache`, which gets previously fetched (ie "cached") items.
/// 
/// The non-cached item IDs are individually scraped.
/// 
/// These scraped items are returned through another Axum endpoint, and fed back into the module as a `SendScrapedItems` message.
/// 
/// Finally, the cached and newly scraped items are combined in the `FinalProcessor`, and sent to the next stage.
pub struct ScraperModule {
    msg_receiver: ScraperReceiver,
    img_analysis_msg_sender: ImgAnalysisSender,
    search_scraper: SearchScraper,
    individual_scraper: IndividualItemsScraper,
    output_processor: OutputProcessor,
    item_cache: Arc<Mutex<ItemCache>>,
    galleries_in_progress: Vec<GalleryScrapingState>,
}

impl ScraperModule {
    /// Initializes the module.
    pub fn init(
        config: ScraperConfig,
        msg_receiver: ScraperReceiver,
        item_storage_msg_sender: MarketplaceItemsStorageSender,
        img_analysis_msg_sender: ImgAnalysisSender,
    ) -> Self
    {   
        let item_cache = Arc::new(Mutex::new(ItemCache::new(item_storage_msg_sender)));
        let search_scraper = SearchScraper::new(config.clone());
        let individual_scraper = IndividualItemsScraper::new(config);
        let output_processor = OutputProcessor::new(item_cache.clone(), img_analysis_msg_sender.clone());
        let galleries_in_progress = Vec::new();

        Self { 
            msg_receiver, 
            img_analysis_msg_sender,
            search_scraper,
            individual_scraper,
            output_processor,
            item_cache,
            galleries_in_progress
        }
    }
    
    /// Start accepting and acting on messages.
    pub async fn run(&mut self) {
        while let Some(msg) = self.msg_receiver.receive().await {
            self.process_msg(msg);
        }
    }

    /// Handle each message variant.
    async fn process_msg(&mut self, msg: ScraperMessage) {
        match msg {
            ScraperMessage::StartScraping(msg) => {
                // TODO: implement a way to Err if gallery is already In Progress, and add it to that list if not
                // TODO: log errors either here, or when returned to a handler (quite critical if there's issues)
                let new_gallery = msg.get_msg().gallery;
                if self.galleries_in_progress
                    .iter()
                    .find(|g| g.gallery_id == new_gallery.gallery_id)
                    .is_some() {
                    todo!() // TODO: respond with ScraperError if this gallery is already in progress
                }
                match self.search_scraper.schedule_scrape_search(new_gallery.clone()).await {
                    Ok(_) => {
                        self.galleries_in_progress.push(new_gallery);
                        msg.respond(Ok(()));
                    },
                    Err(_) => todo!(),
                };
            },
            ScraperMessage::ScrapeIndividualItems(msg) => {
                let inner_msg = msg.get_msg();
                match self.individual_scraper.schedule_scrape_items(
                    inner_msg.gallery_id, 
                    inner_msg.marketplace, 
                    inner_msg.scraped_item_ids
                ).await {
                    Ok(_) => msg.respond(Ok(())),
                    Err(_) => todo!(),
                };
            },
            ScraperMessage::ProcessScrapedItems(msg) => {
                // TODO: implement a way to remove a gallery from In Progress if this is its final marketplace
                // TODO: log errors either here, or when returned to a handler (quite critical if there's issues)
                let inner_msg = msg.get_msg();
                self.output_processor.process_scraped_items(
                    inner_msg.gallery_id.clone(),
                    inner_msg.marketplace,
                    inner_msg.scraped_items
                    ).await;
                let cur_gallery = match self.galleries_in_progress
                    .iter()
                    .find(|g| g.gallery_id == inner_msg.gallery_id) {
                        Some(gallery) => gallery,
                        None => todo!(), // TODO: Return the ScraperError for non-existent gallery here
                    };
                
                // If we've already scraped all required marketplaces for this gallery, remove it from `galleries_in_progress` and send this gallery's items to the next stage.
                let cur_scraped_marketplaces = self.output_processor.get_scraped_marketplaces(&inner_msg.gallery_id);
                if cur_scraped_marketplaces == cur_gallery.marketplaces.iter().collect() {
                    let cur_gallery_pos = self.galleries_in_progress
                        .iter()
                        .position(|g| g.gallery_id != inner_msg.gallery_id)
                        .unwrap(); // This is fine as we ensured this existed above
                    let cur_gallery = self.galleries_in_progress
                        .remove(cur_gallery_pos);
                    self.output_processor
                        .send_gallery_items(cur_gallery.gallery_id, cur_gallery.evaluation_criteria)
                        .await;
                }
                msg.respond(Ok(()));
            },
        }
    }
}