use crate::{domain::{domain_types::GalleryId, pipeline_states::{GalleryFinalState, GalleryPipelineStateTypes, GalleryPipelineStates, GallerySearchScrapingState}}, messages::{message_types::storage::StorageError, StateTrackerSender}, stores::gallery_sessions::GallerySessionsStore};

pub(super) struct Handler {
    state_tracker_sender: StateTrackerSender,
    gallery_sessions_store: GallerySessionsStore
}

impl Handler {
    /// Initialize the handler.
    pub fn new(
        state_tracker_sender: StateTrackerSender, 
        gallery_sessions_store: GallerySessionsStore
    ) -> Self {
        Self {
            state_tracker_sender,
            gallery_sessions_store
        }
    }

    /// Store a gallery in state.
    pub async fn store_gallery_in_state(&mut self, gallery_id: GalleryId) -> Result<(), StorageError> {
        let gallery = self.fetch_gallery_state(gallery_id).await?;
        self.store_gallery(gallery).await
    }

    /// Store a new gallery.
    pub async fn store_gallery(&mut self, state: GalleryFinalState) -> Result<(), StorageError> {
        let gallery_id = state.gallery_id.clone();

        let session_id = self.gallery_sessions_store
            .add_new_session(state)
            .await?;

        tracing::info!("Successfully stored new session for gallery {}; ID: {}", gallery_id, session_id);
        
        Ok(())
    }

    /// Fetches a gallery from state.
    /// 
    /// Returns an `Err` if:
    /// - the gallery is not in state/is in the wrong state/has already been taken 
    /// - the state tracker is not contactable
    async fn fetch_gallery_state(&mut self, gallery_id: GalleryId) -> Result<GalleryFinalState, StorageError> {
        let state = self.state_tracker_sender
            .get_gallery_state(gallery_id.clone(), GalleryPipelineStateTypes::Final)
            .await
            .map_err(|err| StorageError::Other { 
                gallery_id: gallery_id.clone(), 
                message: format!("Could not receive response from state tracker: {err}") 
            })?
            .map_err(|err| StorageError::StateErr { 
                gallery_id: gallery_id.clone(), 
                err 
            })?;
        match state {
            GalleryPipelineStates::Final(gallery_state) => Ok(gallery_state),
            _ => Err(
                StorageError::Other { 
                        gallery_id: gallery_id.clone(), 
                        message: "Gallery is not in expected state".into() 
                    }
                )
        }
    }
}