mod components;

use components::state_manager::StateManager;
use crate::{config::ScraperConfig, messages::{
    message_types::scraper::ScraperMessage, ItemAnalysisSender, MarketplaceItemsStorageSender, ScraperReceiver, StateTrackerSender
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
        state_tracker_msg_sender: StateTrackerSender,
        item_storage_msg_sender: MarketplaceItemsStorageSender,
        img_analysis_msg_sender: ItemAnalysisSender,
    ) -> Self
    {   
        let state_manager = StateManager::new(
            &config, 
            state_tracker_msg_sender,
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
    async fn process_msg(&mut self, msg: ScraperMessage) {
        match msg {
            ScraperMessage::StartScrapingGallery{ gallery } => {
                tracing::trace!("Received message to start scraping gallery {}", gallery.gallery_id);
                let schedule_result = self.state_manager
                    .start_scraping_gallery(gallery)
                    .await;
                if let Err(err) = schedule_result {
                    tracing::error!("Error(s) scheduling scraping tasks ({err:#?})");
                };
            },
        }
    }
}