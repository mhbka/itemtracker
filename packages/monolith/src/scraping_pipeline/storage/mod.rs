use crate::{messages::{message_types::storage::StorageMessage, StateTrackerSender, StorageReceiver}, stores::gallery_sessions::GallerySessionsStore};
use handler::Handler;

mod handler;

/// In charge of scraping the search of marketplaces under a gallery, for item IDs.
pub struct StorageModule {
    msg_receiver: StorageReceiver,
    handler: Handler,
}

impl StorageModule {
    /// Initialize the module.
    pub fn init(
        msg_receiver: StorageReceiver,
        state_tracker_sender: StateTrackerSender,
        gallery_sessions_store: GallerySessionsStore
    ) -> Self
    {   
        let handler = Handler::new(
            state_tracker_sender,
            gallery_sessions_store
        );
        Self { 
            msg_receiver, 
            handler
        }
    }
    
    /// Start accepting and acting on messages.
    pub async fn run(&mut self) {
        tracing::info!("StorageModule is running...");
        while let Some(msg) = self.msg_receiver.receive().await {
            self.process_msg(msg).await;
        }
    }

    /// Handle each message variant.
    async fn process_msg(&mut self, msg: StorageMessage) {
        match msg {
            StorageMessage::StoreGalleryNew{ gallery } => {
                tracing::info!("Received message to store new gallery {}", gallery.gallery_id);
                let schedule_result = self.handler
                    .store_gallery(gallery)
                    .await;
                if let Err(err) = schedule_result {
                    tracing::error!("Error while storing: {err}");
                };
            },
            StorageMessage::StoreGallery{ gallery_id } => {
                tracing::info!("Received message to store gallery {}", gallery_id);
                let schedule_result = self.handler
                    .store_gallery_in_state(gallery_id)
                    .await;
                if let Err(err) = schedule_result {
                    tracing::error!("Error while storing: {err}");
                };
            }
            StorageMessage::StoreGalleryError { gallery_id, error } => {
                tracing::info!("Received message to store error for gallery {gallery_id} (error: {error})");
                todo!()
            }
        }
    }
}