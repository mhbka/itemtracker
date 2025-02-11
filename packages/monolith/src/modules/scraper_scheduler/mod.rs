mod components;
mod msg_handler;

use std::sync::Arc;
use components::scheduler::RawScraperScheduler;
use tokio::sync::Mutex;
use tracing::{info, instrument};
use crate::{config::ScraperSchedulerConfig, messages::{message_types::scraper_scheduler::SchedulerMessage, ScraperSchedulerReceiver, ScraperSender}};

/// Module in charge of scheduling scraping tasks.
/// 
/// This module is fairly straightforward. Gallery creation/update/deletion is received through `msg_receiver`.
/// 
/// Whenever a gallery is scheduled to be scraped, it is sent through the `scraper_msg_sender`.
pub struct ScraperSchedulerModule {
    scheduler: RawScraperScheduler,
    msg_receiver: ScraperSchedulerReceiver,
    scraper_msg_sender: Arc<Mutex<ScraperSender>>
}

impl ScraperSchedulerModule {
    /// Initializes the module.
    pub fn init( 
        config: ScraperSchedulerConfig,
        msg_receiver: ScraperSchedulerReceiver,
        scraper_msg_sender: ScraperSender
    ) -> Self
    {
        let scraper_msg_sender = Arc::new(Mutex::new(scraper_msg_sender));
        let scheduler = RawScraperScheduler::new(scraper_msg_sender.clone());
        ScraperSchedulerModule {
            scheduler,
            msg_receiver,
            scraper_msg_sender
        }
        
    }
    
    /// Start accepting and acting on messages.
    pub async fn run(&mut self) {
        info!("ScraperSchedulerModule is running...");
        while let Some(msg) = self.msg_receiver.receive().await {
            self.process_msg(msg).await;
        }
    }

    /// Handle each message variant.
    async fn process_msg(&mut self, msg: SchedulerMessage) {
        match msg {
            SchedulerMessage::NewGallery(msg) => {
                msg_handler::handle_new_gallery_msg(msg, self).await;
            },
            SchedulerMessage::DeleteGallery(msg) => {
                msg_handler::handle_delete_gallery_msg(msg, self).await;
            },
            SchedulerMessage::EditGallery(msg) => {
                msg_handler::handle_edit_gallery_msg(msg, self).await;
            },
        }
    }
}