use serde::{Deserialize, Serialize};
use crate::{domain::{domain_types::GalleryId, pipeline_states::GalleryFinalState}, stores::error::StoreError};
use thiserror::Error;

use super::state_tracker::StateTrackerError;

/// Possible errors emitted from the scraper.
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Error from state tracker for gallery {gallery_id}: {err}")]
    StateErr { gallery_id: GalleryId, err: StateTrackerError },
    #[error("{0}")]
    StoreErr(#[from] StoreError),
    #[error("Encountered a different error for gallery {gallery_id}: {message}")]
    Other { gallery_id: GalleryId, message: String }
}


/// The messages the storage module can take.
#[derive(Debug)]
pub enum StorageMessage {
    /// Stores a gallery in state, removing it from the state.
    /// If the gallery isn't in state, an error is logged and nothing happens.
    StoreGallery { gallery_id: GalleryId },
    /// Stores a new gallery.
    /// If the gallery is already in state, an error is logged and nothing happens.
    StoreGalleryNew { gallery: GalleryFinalState },
    /// Stores a gallery in state which encountered an error.
    /// If the gallery isn't in state, an error is logged and nothing happens.
    /// TODO: make the error an enum so it can be logged properly?
    StoreGalleryError { gallery_id: GalleryId, error: String } 
}