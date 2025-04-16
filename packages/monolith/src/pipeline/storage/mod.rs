use error::StorageError;

use crate::{domain::{gallery_session::SessionId, pipeline_states::GalleryFinalState}, stores::gallery_sessions::GallerySessionsStore};

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

    /// Store the final state of the pipeline, returning the new session ID.
    pub async fn store(&mut self, gallery_state: GalleryFinalState) -> Result<SessionId, StorageError> {
        self.log_gallery(&gallery_state);

        let gallery_id = gallery_state.gallery_id;
        let session_id = self.gallery_sessions_store
            .add_new_session(gallery_state)
            .await?;

        tracing::info!("Successfully stored new session for gallery {}; ID: {}", gallery_id, session_id);

        Ok(session_id)
    }

    /// Write some useful stats about the gallery to logging.
    fn log_gallery(&self, gallery_state: &GalleryFinalState) {
        let mut log_string = format!("Final statistics for gallery {}: ", gallery_state.gallery_id);

        for (marketplace, items) in &gallery_state.items {
            let item_log = format!(
                "- {}\n -- {} embedded\n -- {} irrelevant analyzed\n -- {} error analyzed\n -- {} error embedded\n",
                marketplace,
                items.embedded_items.len(),
                items.irrelevant_analyzed_items.len(),
                items.error_analyzed_items.len(),
                items.error_embedded_items.len(),
            );
            log_string += &item_log;
        }
        log_string += &format!("- Updated datetimes: {:?}", gallery_state.marketplace_updated_datetimes);
        log_string += &format!("- Failure reasons: {:?}", gallery_state.failed_marketplace_reasons);

        tracing::debug!(log_string);
    }
}