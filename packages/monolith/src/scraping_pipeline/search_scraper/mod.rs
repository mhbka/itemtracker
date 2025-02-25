use handler::Handler;
use crate::{config::SearchScraperConfig, messages::{
    message_types::search_scraper::SearchScraperMessage, ItemScraperSender, SearchScraperReceiver, StateTrackerSender
}};

mod handler;
mod scrapers;

/// In charge of scraping the search of marketplaces under a gallery, for item IDs.
pub struct SearchScraperModule {
    msg_receiver: SearchScraperReceiver,
    handler: Handler,
}

impl SearchScraperModule {
    /// Initialize the module.
    pub fn init(
        config: SearchScraperConfig,
        msg_receiver: SearchScraperReceiver,
        state_tracker_msg_sender: StateTrackerSender,
        item_scraper_msg_sender: ItemScraperSender
    ) -> Self
    {   
        let handler = Handler::new(
            &config, 
            state_tracker_msg_sender,
            item_scraper_msg_sender
        );
        Self { 
            msg_receiver, 
            handler
        }
    }
    
    /// Start accepting and acting on messages.
    pub async fn run(&mut self) {
        tracing::info!("SearchScraperModule is running...");
        while let Some(msg) = self.msg_receiver.receive().await {
            self.process_msg(msg).await;
        }
    }

    /// Handle each message variant.
    async fn process_msg(&mut self, msg: SearchScraperMessage) {
        match msg {
            SearchScraperMessage::ScrapeSearchNew{ gallery } => {
                tracing::info!("Received message to start search-scraping new gallery {}", gallery.gallery_id);
                let schedule_result = self.handler
                    .scrape_new_gallery(gallery)
                    .await;
                if let Err(err) = schedule_result {
                    tracing::error!("Error while search scraping: {err}");
                };
            },
            SearchScraperMessage::ScrapeSearch{ gallery_id } => {
                tracing::info!("Received message to start scraping gallery {}", gallery_id);
                let schedule_result = self.handler
                    .scrape_gallery_in_state(gallery_id)
                    .await;
                if let Err(err) = schedule_result {
                    tracing::error!("Error while search scraping: {err}");
                };
            }
        }
    }
}