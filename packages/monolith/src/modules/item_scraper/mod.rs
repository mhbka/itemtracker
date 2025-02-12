use crate::messages::{message_types::item_scraper::ItemScraperMessage, ItemScraperReceiver};

mod handler;
mod scrapers;

pub struct ItemScraperModule {
    msg_receiver: ItemScraperReceiver
}

impl ItemScraperModule {
    /// Initialize the module.
    pub fn init(msg_receiver: ItemScraperReceiver) -> Self {
        Self {
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
            ItemScraperMessage::ScrapeItems { gallery } => {

            },
        }
    }
}