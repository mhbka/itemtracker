mod components;
mod msg_handler;

use components::state_manager::StateManager;

use crate::{config::ScraperConfig, galleries::{domain_types::GalleryId, scraping_pipeline::GalleryScrapingState}, messages::{
    message_types::scraper::ScraperMessage, ImgAnalysisSender, MarketplaceItemsStorageSender, ScraperReceiver
}};


/// Module in charge of orchestrating the actual scraping through Scrapy spiders.
/// 
/// The flow of this module is rather complex:
/// 
/// - Scraping begins from a `StartScrapingGallery` message.
/// For each contained marketplace, a scrape of its search is scheduled through the `SearchScraper`, 
/// using the contained search criteria.
/// 
/// - As each marketplace search scrape completes, its data sent back through a HTTP endpoint in this app,
/// which feeds (mostly scraped item IDs) back into this module as a `IngestScrapedSearch` message.
/// 
/// - These item IDs are sent to the item storage module through the `ItemCache`, which gets previously fetched (ie "cached") items.
/// For non-cached item IDs, a scrape of its detailed data is scheduled through the `ItemScraper`.
/// 
/// - As each item scrape completes, its data is sent back through another HTTP endpoint in this app,
/// which feeds back into this module as a `IngestScrapedItems` message.
/// 
/// - This item data is combined with the `ItemCache`'s cached data inside the `OutputProcessor`. 
/// Here, it waits until all marketplaces for the gallery have been item-scraped.
/// 
/// - Once this happens, all data is combined for the gallery and sent to the next module.
pub struct ScraperModule {
    msg_receiver: ScraperReceiver,
    state_manager: StateManager,
}

impl ScraperModule {
    /// Initialize the module.
    pub fn init(
        config: ScraperConfig,
        msg_receiver: ScraperReceiver,
        item_storage_msg_sender: MarketplaceItemsStorageSender,
        img_analysis_msg_sender: ImgAnalysisSender,
    ) -> Self
    {   
        let state_manager = StateManager::new(
            &config, 
            item_storage_msg_sender, 
            img_analysis_msg_sender
        );
        Self { 
            msg_receiver, 
            state_manager
        }
    }
    
    /// Start accepting and acting on messages.
    pub async fn run(&mut self) {
        tracing::info!("ScraperModule is running...");
        while let Some(msg) = self.msg_receiver.receive().await {
            self.process_msg(msg).await;
        }
    }

    /// Handle each message variant.
    #[tracing::instrument(skip(self, msg))]
    async fn process_msg(&mut self, msg: ScraperMessage) {
        match msg {
            ScraperMessage::StartScrapingGallery(msg) => {
                msg_handler::handle_start_scraping_gallery_msg(msg, self).await;
            },
            ScraperMessage::IngestScrapedSearch(msg) => {
                msg_handler::handle_ingest_scraped_search_msg(msg, self).await;
            },
            ScraperMessage::IngestScrapedItems(msg) => {
                msg_handler::handle_ingest_scraped_items_msg(msg, self).await;
            },
        }
    }
}