use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::{domain::{domain_types::GalleryId, pipeline_states::GalleryItemAnalysisState}, messages::message_buses::MessageError};
use super::state_tracker::StateTrackerError;

/// Possible errors emitted from the item analysis module.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum ItemAnalysisError {
    #[error("Error from state tracker for gallery {gallery_id}: {err}")]
    StateErr { gallery_id: GalleryId, err: StateTrackerError },
    #[error("Error while sending a message for gallery {gallery_id}: {err}")]
    MessageErr { gallery_id: GalleryId, err: MessageError },
    #[error("Encountered an different error for gallery {gallery_id}: {message}")]
    Other { gallery_id: GalleryId, message: String }
}

/// The types of messages the item analysis module can take.
#[derive(Debug)]
pub enum ItemAnalysisMessage {
    /// Message for starting analysis of a gallery in state.
    /// If the gallery isn't in state, an error is logged and nothing happens.
    AnalyzeGallery { gallery_id: GalleryId },
    /// Message for starting analysis of a new gallery.
    AnalyzeGalleryNew { gallery: GalleryItemAnalysisState },
}