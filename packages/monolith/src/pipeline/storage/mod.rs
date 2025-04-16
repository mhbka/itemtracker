use error::StorageError;

use crate::{domain::pipeline_states::GalleryFinalState, stores::gallery_sessions::GallerySessionsStore};

pub mod error;

/// Handles storage of pipeline states.
#[derive(Clone)]
pub struct Storage {
    gallery_sessions_store: GallerySessionsStore
}

impl Storage {
    /// Initialize the pipeline storage.
    pub fn new(gallery_sessions_store: GallerySessionsStore) -> Self {
        Self {
            gallery_sessions_store
        }
    }

    /// Store the final state of the pipeline.
    pub async fn store(&mut self, gallery_state: GalleryFinalState) -> Result<(), StorageError> {
        let gallery_id = gallery_state.gallery_id;
        let session_id = self.gallery_sessions_store
            .add_new_session(gallery_state)
            .await?;

        tracing::info!("Successfully stored new session for gallery {}; ID: {}", gallery_id, session_id);

        Ok(())
    }
}