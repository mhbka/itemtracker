use handler::Handler;
use crate::{config::ItemAnalysisConfig, messages::{message_types::item_analysis::ItemAnalysisMessage, ItemEmbedderSender, ItemAnalysisReceiver, StateTrackerSender}};

mod handler;
mod analyzer;

/// Module in charge of orchestrating analysis of scraped items.
/// 
/// At its core, it just calls out to an LLM with the item information + images;
/// the rest is just error handling/data parsing/other administration.
pub struct ItemAnalysisModule {
    config: ItemAnalysisConfig,
    msg_receiver: ItemAnalysisReceiver,
    handler: Handler
}

impl ItemAnalysisModule {
    /// Initialize the module.
    pub fn init(
        config: ItemAnalysisConfig, 
        msg_receiver: ItemAnalysisReceiver,
        state_tracker_sender: StateTrackerSender,
        image_classifier_sender: ItemEmbedderSender
    ) -> Self {
        let handler = Handler::new(
            &config, 
            state_tracker_sender, 
            image_classifier_sender
        );
        Self { 
            config,
            msg_receiver,
            handler
        }
    }

    /// Start accepting and handling messages.
    pub async fn run(&mut self) {
        tracing::info!("ItemAnalysisModule is running...");
        while let Some(msg) = self.msg_receiver.receive().await {
            self.process_msg(msg).await;
        }
    }

    /// Handle each message variant.
    async fn process_msg(&mut self, msg: ItemAnalysisMessage) {
        match msg {
            ItemAnalysisMessage::AnalyzeGallery { gallery_id } => {
                tracing::trace!("Received message to start analyzing gallery {gallery_id} in state");
                let schedule_result = self.handler
                    .analyze_gallery_in_state(gallery_id)
                    .await;
                if let Err(err) = schedule_result {
                    tracing::error!("Error(s) performing analysis ({err:#?})");
                };
            },
            ItemAnalysisMessage::AnalyzeGalleryNew { gallery } => {
                tracing::trace!("Received message to start analyzing new gallery {}", gallery.gallery_id);
                let schedule_result = self.handler
                    .analyze_new_gallery(gallery)
                    .await;
                if let Err(err) = schedule_result {
                    tracing::error!("Error(s) performing analysis ({err:#?})");
                };
            }
        }
    }
}