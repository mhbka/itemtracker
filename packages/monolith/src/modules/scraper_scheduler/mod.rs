use scheduler::SchedulerHandler;
use tracing::info;
use crate::{config::ScraperSchedulerConfig, messages::{message_types::scraper_scheduler::SchedulerMessage, ScraperSchedulerReceiver, SearchScraperSender, StateTrackerSender}};

mod scheduled_task;
mod scheduler;

/// Module in charge of scheduling scraping tasks.
/// 
/// This module is fairly straightforward. Gallery creation/update/deletion is received through `msg_receiver`.
/// 
/// Whenever a gallery is scheduled to be scraped, it is sent through the `search_scraper_sender`.
pub struct ScraperSchedulerModule {
    scheduler: SchedulerHandler,
    msg_receiver: ScraperSchedulerReceiver,
    search_scraper_sender: SearchScraperSender
}

impl ScraperSchedulerModule {
    /// Initializes the module.
    pub fn init( 
        config: ScraperSchedulerConfig,
        msg_receiver: ScraperSchedulerReceiver,
        search_scraper_sender: SearchScraperSender,
        state_tracker_sender: StateTrackerSender
    ) -> Self
    {
        ScraperSchedulerModule {
            scheduler: SchedulerHandler::new(search_scraper_sender.clone(), state_tracker_sender),
            msg_receiver,
            search_scraper_sender
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
                if msg.act_async(|gallery| async {
                    self.scheduler.add_gallery(gallery).await
                })
                    .await
                    .is_err() {
                        tracing::error!("Was unable to respond to a message for creating a gallery");
                    };
            },
            SchedulerMessage::DeleteGallery(msg) => {
                if msg.act_async(|gallery_id| async {
                    self.scheduler.delete_gallery(gallery_id).await
                })
                    .await
                    .is_err() {
                        tracing::error!("Was unable to respond to a message for creating a gallery");
                    };
            },
            SchedulerMessage::UpdateGallery(msg) => {
                if msg.act_async(|gallery| async {
                    self.scheduler.update_gallery(gallery).await
                })
                    .await
                    .is_err() {
                        tracing::error!("Was unable to respond to a message for creating a gallery");
                    };
            },
        }
    }
}