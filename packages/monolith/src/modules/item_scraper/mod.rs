use handler::Handler;
use crate::{config::ItemScraperConfig, messages::{message_types::item_scraper::ItemScraperMessage, ItemAnalysisSender, ItemScraperReceiver, StateTrackerSender}};

mod handler;
mod scrapers;

pub struct ItemScraperModule {
    handler: Handler,
    msg_receiver: ItemScraperReceiver
}

impl ItemScraperModule {
    /// Initialize the module.
    pub fn init(
        config: ItemScraperConfig, 
        msg_receiver: ItemScraperReceiver,
        state_tracker_sender: StateTrackerSender,
        item_analysis_sender: ItemAnalysisSender
    ) -> Self {
        let handler = Handler::new(
            &config, 
            state_tracker_sender, 
            item_analysis_sender
        );
        Self {
            handler,
            msg_receiver
        }
    }

    /// Start accepting and acting on messages.
    pub async fn run(&mut self) {
        tracing::info!("ItemScraperModule is running...");
        while let Some(msg) = self.msg_receiver.receive().await {
            self.process_msg(msg).await;
        }
    }

    /// Handle each message variant.
    async fn process_msg(&mut self, msg: ItemScraperMessage) {
        match msg {
            ItemScraperMessage::ScrapeItems { gallery_id } => {
                tracing::trace!("Received message to start item scraping gallery {} in state", gallery_id);
                let schedule_result = self.handler
                    .scrape_gallery_in_state(gallery_id)
                    .await;
                if let Err(err) = schedule_result {
                    tracing::error!("Error(s) scheduling scraping tasks ({err:#?})");
                };
            },
            ItemScraperMessage::ScrapeItemsNew { gallery } => {
                tracing::trace!("Received message to start search-scraping new gallery {}", gallery.gallery_id);
                let schedule_result = self.handler
                    .scrape_new_gallery(gallery)
                    .await;
                if let Err(err) = schedule_result {
                    tracing::error!("Error(s) scheduling scraping tasks ({err:#?})");
                };
            },
        }
    }
}