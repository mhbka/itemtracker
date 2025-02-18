use crate::galleries::pipeline_states::GalleryClassifierState;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::galleries::domain_types::GalleryId;
use super::state_tracker::StateTrackerError;

/// Possible errors emitted from the item analysis module.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum ImgClassifierError {
    #[error("Error from state tracker for gallery {gallery_id}: {err}")]
    StateErr { gallery_id: GalleryId, err: StateTrackerError },
    #[error("Encountered an different error for gallery {gallery_id}: {message}")]
    Other { gallery_id: GalleryId, message: String }
}

/// The types of messages the image classifer module can take.
#[derive(Debug)]
pub enum ImageClassifierMessage {
    Classify { gallery_id: GalleryId },
    ClassifyNew { gallery: GalleryClassifierState }
}