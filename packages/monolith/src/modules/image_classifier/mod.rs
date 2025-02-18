// TODO: https://towardsdatascience.com/building-an-image-similarity-search-engine-with-faiss-and-clip-2211126d08fa 
// this sounds pretty solid

use crate::{config::ImageClassifierConfig, messages::{message_types::img_classifier::ImageClassifierMessage, ImageClassifierReceiver}};

/// This module handles classification of scraped and analyzed items under a gallery.
pub struct ImageClassifierModule {
    config: ImageClassifierConfig,
    msg_receiver: ImageClassifierReceiver
}

impl ImageClassifierModule {
    /// Instantiate the module.
    pub fn init(
        config: ImageClassifierConfig,
        msg_receiver: ImageClassifierReceiver
    ) -> Self {
        Self {
            config,
            msg_receiver
        }
    }
    
    /// Start accepting and handling messages.
    pub async fn run(&mut self) {
        tracing::info!("ImageClassifierModule is running...");
        while let Some(msg) = self.msg_receiver.receive().await {
            self.process_msg(msg).await;
        }
    }

    /// Handle each message variant.
    async fn process_msg(&mut self, msg: ImageClassifierMessage) {
        match msg {
            ImageClassifierMessage::Classify { gallery_id } => {
                todo!();
            },
            ImageClassifierMessage::ClassifyNew { gallery } => {
                todo!();
            },
        }
    }
}