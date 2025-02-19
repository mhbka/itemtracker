use std::collections::HashMap;
use crate::{
    config::ItemEmbedderConfig, 
    galleries::{domain_types::{GalleryId, Marketplace, UnixUtcDateTime}, pipeline_states::{GalleryItemEmbedderState, GalleryItemAnalysisState, GalleryPipelineStateTypes, GalleryPipelineStates}}, 
    messages::{
        message_types::item_embedder::{ItemEmbedderError, ItemEmbedderMessage
        }, ItemEmbedderSender, StateTrackerSender
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
    image_classifier_sender: ItemEmbedderSender,
    embedder: Embedder
}

impl Handler {
    /// Instantiate the state.
    pub fn new(
        config: &ItemEmbedderConfig,
        state_tracker_sender: StateTrackerSender,
        image_classifier_sender: ItemEmbedderSender
    ) -> Self {
        let embedder = Embedder::new(config.clone());
        Self {
            state_tracker_sender,
            image_classifier_sender,
            embedder
        }
    }
    
    /// Embed a new gallery.
    pub async fn embed_new_gallery(&mut self, gallery: GalleryItemAnalysisState) -> Result<(), ItemEmbedderError> {
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
    async fn embed_gallery(&mut self, gallery: GalleryItemAnalysisState) -> Result<(), ItemEmbedderError> {
        let embedd_items = self.embedder
            .embed_gallery(gallery.items, &gallery.evaluation_criteria)
            .await;
        let gallery_id = gallery.gallery_id.clone();
        self.update_gallery_state(
            gallery.gallery_id,
            embedd_items,
            gallery.marketplace_updated_datetimes,
            gallery.failed_marketplace_reasons,
        ).await?;
        self.image_classifier_sender
            .send(ItemEmbedderMessage::Classify { gallery_id })
            .await;
            Ok(())
    }
    
    /// Add a new gallery to the state.
    /// 
    /// Returns an `Err` if it already exists.
    async fn add_gallery_to_state(
        &mut self, 
        gallery_id: GalleryId, 
        gallery: GalleryItemAnalysisState
    ) -> Result<(), ItemEmbedderError> {
        self.state_tracker_sender
            .add_gallery(gallery_id.clone(), GalleryPipelineStates::ItemAnalysis(gallery))
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
    async fn fetch_gallery_state(&mut self, gallery_id: GalleryId) -> Result<GalleryItemAnalysisState, ItemEmbedderError> {
        let state = self.state_tracker_sender
            .take_gallery_state(gallery_id.clone(), GalleryPipelineStateTypes::SearchScraping)
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
            GalleryPipelineStates::ItemAnalysis(gallery_state) => Ok(gallery_state),
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
        embedded_items: HashMap<Marketplace, MarketplaceAnalyzedItems>,
        marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
        failed_marketplace_reasons: HashMap<Marketplace, String>,
    ) -> Result<(), ItemEmbedderError> {
        let new_state = self.process_to_next_state(
            gallery_id.clone(), 
            embedd_items, 
            marketplace_updated_datetimes, 
            failed_marketplace_reasons
        );
        self.state_tracker_sender
            .update_gallery_state(
                gallery_id.clone(), 
                GalleryPipelineStates::Classification(new_state)
            )
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
        embedd_items: HashMap<Marketplace, MarketplaceembeddItems>,
        marketplace_updated_datetimes: HashMap<Marketplace, UnixUtcDateTime>,
        failed_marketplace_reasons: HashMap<Marketplace, String>,
    ) -> GalleryItemEmbedderState {
        GalleryItemEmbedderState {
            gallery_id,
            items: embedd_items,
            marketplace_updated_datetimes,
            failed_marketplace_reasons,
        }
    }
}
