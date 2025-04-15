use scheduler::SchedulerHandler;
use tracing::info;
use crate::{config::ScraperSchedulerConfig, domain::pipeline_states::GallerySchedulerState, messages::{message_types::scraper_scheduler::SchedulerMessage, ScraperSchedulerReceiver, SearchScraperSender, StateTrackerSender}};

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
}

impl ScraperSchedulerModule {
    /// Initializes the module.
    pub async fn init( 
        config: ScraperSchedulerConfig,
        initial_state: Vec<GallerySchedulerState>,
        msg_receiver: ScraperSchedulerReceiver,
        search_scraper_sender: SearchScraperSender,
        state_tracker_sender: StateTrackerSender
    ) -> Self
    {   
        let scheduler = SchedulerHandler::new(
            search_scraper_sender, 
            state_tracker_sender,
            initial_state
        ).await;

        ScraperSchedulerModule {
            scheduler,
            msg_receiver,
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
                let result = msg.act_async(|gallery| async {
                    tracing::info!("Received message to add gallery {} to scheduler", gallery.gallery_id);
                    self.scheduler.add_gallery(gallery).await
                })
                    .await;
                if let Err(err) = result {
                    tracing::error!("Could not respond to message; response: {err:?}");
                }
            },
            SchedulerMessage::DeleteGallery(msg) => {
                let result = msg.act_async(|gallery_id| async {
                    tracing::info!("Received message to delete gallery {gallery_id} from scheduler");
                    self.scheduler.delete_gallery(gallery_id).await
                })
                    .await;
                if let Err(err) = result {
                    tracing::error!("Could not respond to message; response: {err:?}");
                };
            },
            SchedulerMessage::UpdateGallery(msg) => {
                let result = msg.act_async(|gallery| async {
                    tracing::info!("Received message to delete gallery {} in scheduler", gallery.gallery_id);
                    self.scheduler.update_gallery(gallery).await
                })
                    .await;
                if let Err(err) = result {
                    tracing::error!("Could not respond to message; response: {err:?}");
                };
            },
        }
    }
}