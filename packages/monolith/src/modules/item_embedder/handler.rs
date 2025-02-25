use std::collections::HashMap;
use crate::{
    config::ItemEmbedderConfig, 
    galleries::{domain_types::{GalleryId, Marketplace, UnixUtcDateTime}, items::pipeline_items::MarketplaceEmbeddedAndAnalyzedItems, pipeline_states::{GalleryFinalState, GalleryItemEmbedderState, GalleryPipelineStateTypes, GalleryPipelineStates}}, 
    messages::{
        message_types::{item_embedder::ItemEmbedderError, storage::StorageMessage}, StateTrackerSender, StorageSender
    }
};

use super::embedder::Embedder;

/*
TODO:
- Add to analysis:
    - "Briefly and accurately describe this item"
    - "Pick the image which best describes this item/shows the most recognizable feature of this item" (make sure only uint)
- Download chosen image of each item
- Create POST request with descriptions + image data, in order
- POST to the endpoint in the config
- Parse the embeddings back to corresponding items
- Pass to next module

Might be better to change the name to "image embedder". Then we pass embeddings out 
and have the actual classification done elsewhere, where there's more explicit access
to the gallery's past data.

^Note: Classification done on some mix of cosine similarity for description + image
*/

/// Coordinates the internal workings of the module.
pub(super) struct Handler {
    state_tracker_sender: StateTrackerSender,
    storage_sender: StorageSender,
    embedder: Embedder
}

impl Handler {
    /// Instantiate the state.
    pub fn new(
        config: &ItemEmbedderConfig,
        state_tracker_sender: StateTrackerSender,
        storage_sender: StorageSender
    ) -> Self {
        let embedder = Embedder::new(config.clone());
        Self {
            state_tracker_sender,
            storage_sender,
            embedder
        }
    }
    
    /// Embed a new gallery.
    pub async fn embed_new_gallery(&mut self, gallery: GalleryItemEmbedderState) -> Result<(), ItemEmbedderError> {
        let gallery_id = gallery.gallery_id.clone();
        self.add_gallery_to_state(gallery_id.clone(), gallery).await?;
        self.embed_gallery_in_state(gallery_id).await
    }

    /// Embed items of a gallery in state.
    pub async fn embed_gallery_in_state(&mut self, gallery_id: GalleryId) -> Result<(), ItemEmbedderError> {
        let gallery = self.fetch_gallery_state(gallery_id).await?;
        self.embed_gallery(gallery).await
    }

    /// Embed a gallery's items' descriptions + images and send it to the next stage.
    async fn embed_gallery(&mut self, gallery: GalleryItemEmbedderState) -> Result<(), ItemEmbedderError> {
        let embedded_items = self.embedder
            .embed_gallery(gallery.items)
            .await;
        let gallery_id = gallery.gallery_id.clone();
        self.update_gallery_state(
            gallery.gallery_id,
            embedded_items,
            gallery.marketplace_updated_datetimes,
            gallery.failed_marketplace_reasons,
        ).await?;
        self.storage_sender
            .send(StorageMessage::StoreGallery { gallery_id: gallery_id.clone() })
            .await
            .map_err(|err| ItemEmbedderError::MessageErr { gallery_id, err })?;
            Ok(())
    }
    
    /// Add a new gallery to the state.
    /// 
    /// Returns an `Err` if it already exists.
    async fn add_gallery_to_state(
        &mut self, 
        gallery_id: GalleryId, 
        gallery: GalleryItemEmbedderState
    ) -> Result<(), ItemEmbedderError> {
        self.state_tracker_sender
            .add_gallery(gallery_id.clone(), GalleryPipelineStates::ItemEmbedding(gallery))
            .await
            .map_err(|err| ItemEmbedderError::Other { 
                gallery_id: gallery_id.clone(), 
                message: format!("Could not receive response from state tracker: {err}") 
            })?
            .map_err(|err| ItemEmbedderError::StateErr { 
                gallery_id, 
                err 
            })
    }
    /// Fetches a gallery from state.
    /// 
    /// Returns an `Err` if:
    /// - the gallery is not in state/is in the wrong state/has already been taken 
    /// - the state tracker is not contactable
    async fn fetch_gallery_state(&mut self, gallery_id: GalleryId) -> Result<GalleryItemEmbedderState, ItemEmbedderError> {
        let state = self.state_tracker_sender
            .get_gallery_state(gallery_id.clone(), GalleryPipelineStateTypes::ItemEmbedding)
            .await
            .map_err(|err| ItemEmbedderError::Other { 
                gallery_id: gallery_id.clone(),
                message: format!("Could not receive response from state tracker: {err}") 
            })?
            .map_err(|err| ItemEmbedderError::StateErr { 
                gallery_id: gallery_id.clone(), 
                err
            })?;
        match state {
            GalleryPipelineStates::ItemEmbedding(gallery_state) => Ok(gallery_state),
            _ => Err(
                    ItemEmbedderError::Other { 
                        gallery_id: gallery_id.clone(), 
                        message: "Gallery is not in expected state".into()
                    }
                )
        }
    }

    /// Updates the state for a search-scraped gallery.
    /// 
    /// Returns an `Err` if:
    /// - all marketplaces failed to scrape (also removing the gallery from state),
    /// - the gallery is not in state/is in the wrong state/has already been taken,
    /// - the state tracker module couldn't be contacted.
    async fn update_gallery_state(
        &mut self, 
        gallery_id: GalleryId,
        embedded_items: HashMap<Marketplace, MarketplaceEmbeddedAndAnalyzedItems>,
        marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
        failed_marketplace_reasons: HashMap<Marketplace, String>,
    ) -> Result<(), ItemEmbedderError> {
        let new_state = self.process_to_next_state(
            gallery_id.clone(), 
            embedded_items, 
            marketplace_updated_datetimes, 
            failed_marketplace_reasons
        );
        self.state_tracker_sender
            .update_gallery_state(gallery_id.clone(), GalleryPipelineStates::Final(new_state))
            .await
            .map_err(|err| ItemEmbedderError::Other { 
                gallery_id: gallery_id.clone(), 
                message: format!("Got an error messaging the state tracker: {err}")
            })?
            .map_err(|err| ItemEmbedderError::StateErr { 
                gallery_id, 
                err 
            })
    }

    /// Process the gallery's state into the next state.
    fn process_to_next_state(
        &self,
        gallery_id: GalleryId,
        embedded_items: HashMap<Marketplace, MarketplaceEmbeddedAndAnalyzedItems>,
        marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
        failed_marketplace_reasons: HashMap<Marketplace, String>,
    ) -> GalleryFinalState {
        GalleryFinalState {
            gallery_id,
            items: embedded_items,
            marketplace_updated_datetimes,
            failed_marketplace_reasons,
        }
    }
}
