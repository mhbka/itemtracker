// TODO: https://towardsdatascience.com/building-an-image-similarity-search-engine-with-faiss-and-clip-2211126d08fa 
// this sounds pretty solid

use handler::Handler;

use crate::{config::ItemEmbedderConfig, messages::{message_types::item_embedder::ItemEmbedderMessage, ItemEmbedderReceiver, StateTrackerSender, StorageSender}};

mod handler;
mod embedder;

/// This module handles classification of scraped and analyzed items under a gallery.
pub struct ItemEmbedderModule {
    config: ItemEmbedderConfig,
    msg_receiver: ItemEmbedderReceiver,
    handler: Handler
}

impl ItemEmbedderModule {
    /// Instantiate the module.
    pub fn init(
        config: ItemEmbedderConfig,
        msg_receiver: ItemEmbedderReceiver,
        state_tracker_sender: StateTrackerSender,
        storage_sender: StorageSender
    ) -> Self {
        let handler = Handler::new(
            &config, 
            state_tracker_sender, 
            storage_sender
        );
        Self {
            config,
            msg_receiver,
            handler
        }
    }
    
    /// Start accepting and handling messages.
    pub async fn run(&mut self) {
        tracing::info!("ItemEmbedderModule is running...");
        while let Some(msg) = self.msg_receiver.receive().await {
            self.process_msg(msg).await;
        }
    }

    /// Handle each message variant.
    async fn process_msg(&mut self, msg: ItemEmbedderMessage) {
        match msg {
            ItemEmbedderMessage::Classify { gallery_id } => {
                tracing::trace!("Received message to start embedding gallery {} in state", gallery_id);
                let schedule_result = self.handler
                    .embed_gallery_in_state(gallery_id)
                    .await;
                if let Err(err) = schedule_result {
                    tracing::error!("Error(s) performing item scrape ({err:#?})");
                };
            },
            ItemEmbedderMessage::ClassifyNew { gallery } => {
                tracing::trace!("Received message to start embedding new gallery {}", gallery.gallery_id);
                let schedule_result = self.handler
                    .embed_new_gallery(gallery)
                    .await;
                if let Err(err) = schedule_result {
                    tracing::error!("Error(s) performing item scrape ({err:#?})");
                };
            },
        }
    }
}