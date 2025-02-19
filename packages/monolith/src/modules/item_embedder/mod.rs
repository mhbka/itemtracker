// TODO: https://towardsdatascience.com/building-an-image-similarity-search-engine-with-faiss-and-clip-2211126d08fa 
// this sounds pretty solid

use crate::{config::ItemEmbedderConfig, messages::{message_types::item_embedder::ItemEmbedderMessage, ItemEmbedderReceiver}};

mod handler;
mod embedder;

/// This module handles classification of scraped and analyzed items under a gallery.
pub struct ItemEmbedderModule {
    config: ItemEmbedderConfig,
    msg_receiver: ItemEmbedderReceiver
}

impl ItemEmbedderModule {
    /// Instantiate the module.
    pub fn init(
        config: ItemEmbedderConfig,
        msg_receiver: ItemEmbedderReceiver
    ) -> Self {
        Self {
            config,
            msg_receiver
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
                todo!();
            },
            ItemEmbedderMessage::ClassifyNew { gallery } => {
                todo!();
            },
        }
    }
}