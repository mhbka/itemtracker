use handler::Handler;
use messages::SchedulerMessage;
use tokio::sync::mpsc::Receiver;
use crate::{config::SchedulerConfig, domain::pipeline_states::GallerySchedulerState};

use super::instance::PipelineInstance;

pub mod error;
pub mod messages;
mod scheduled_task;
mod handler;

/// Module in charge of scheduling scraping tasks.
/// 
/// This module is fairly straightforward. Gallery creation/update/deletion is received through `receiver`.
/// 
/// Whenever a gallery is scheduled to be scraped, it is sent through the `search_scraper_sender`.
pub struct Scheduler {
    scheduler: Handler,
    receiver: Receiver<SchedulerMessage>,
}

impl Scheduler {
    /// Initializes the module.
    pub async fn init(
        config: SchedulerConfig, 
        pipeline_instance: PipelineInstance,
        initial_state: Vec<GallerySchedulerState>,
        receiver: Receiver<SchedulerMessage>,
    ) -> Self {   
        let scheduler = Handler::new(initial_state, pipeline_instance).await;
        Scheduler {
            scheduler,
            receiver,
        }
    }
    
    /// Start accepting and acting on messages.
    pub async fn run(&mut self) {
        tracing::info!("Scheduler is running...");
        while let Some(msg) = self.receiver.recv().await {
            self.process_msg(msg).await;
        }
    }

    /// Handle each message variant.
    async fn process_msg(&mut self, msg: SchedulerMessage) {
        match msg {
            SchedulerMessage::AddGallery((state, responder)) => {
                let gallery_id = state.gallery_id;

                tracing::info!("Received message to add gallery {} in scheduler", state.gallery_id);

                let result = self.scheduler.add_gallery(state).await;
                let send_result = responder.send(result);

                if let Err(err) = send_result {
                    tracing::warn!("Was unable to respond to deleted gallery request for gallery {gallery_id} (maybe the caller hung up?)");
                }
                tracing::debug!("Successfully added gallery {gallery_id} in scheduler");
            },
            SchedulerMessage::DeleteGallery((gallery_id, responder)) => {
                tracing::info!("Received message to delete gallery {gallery_id} from scheduler");

                let result = self.scheduler.delete_gallery(gallery_id).await;
                let send_result = responder.send(result);

                if let Err(err) = send_result {
                    tracing::warn!("Was unable to respond to deleted gallery request for gallery {gallery_id} (maybe the caller hung up?)");
                }
                tracing::debug!("Successfully deleted gallery {gallery_id} in scheduler");
            },
            SchedulerMessage::UpdateGallery((state, responder)) => {
                let gallery_id = state.gallery_id;

                tracing::info!("Received message to update gallery {gallery_id} in scheduler");
                
                let result = self.scheduler.update_gallery(state).await;
                let send_result = responder.send(result);

                if let Err(err) = send_result {
                    tracing::warn!("Was unable to respond to deleted gallery request for gallery {gallery_id} (maybe the caller hung up?)");
                }
                tracing::debug!("Successfully updated gallery {gallery_id} in scheduler");
            },
        }
    }
}