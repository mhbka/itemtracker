use crate::galleries::pipeline_states::GalleryItemEmbedderState;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::galleries::domain_types::GalleryId;
use super::state_tracker::StateTrackerError;

/// Possible errors emitted from the item analysis module.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum ItemEmbedderError {
    #[error("Failed to embed any items for gallery {gallery_id}")]
    TotalEmbedFailure { gallery_id: GalleryId },
    #[error("Error from state tracker for gallery {gallery_id}: {err}")]
    StateErr { gallery_id: GalleryId, err: StateTrackerError },
    #[error("Encountered an different error for gallery {gallery_id}: {message}")]
    Other { gallery_id: GalleryId, message: String }
}

/// The types of messages the image classifer module can take.
#[derive(Debug)]
pub enum ItemEmbedderMessage {
    Classify { gallery_id: GalleryId },
    ClassifyNew { gallery: GalleryItemEmbedderState }
}