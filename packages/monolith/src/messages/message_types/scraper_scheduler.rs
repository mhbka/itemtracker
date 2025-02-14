use thiserror::Error;
use serde::{Deserialize, Serialize};
use crate::galleries::{domain_types::GalleryId, pipeline_states::GalleryInitializationState};
use super::ModuleMessageWithReturn;

/// Possible errors emitted from the scraper scheduler.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum SchedulerError {
    /// Emitted when a deleted gallery's ID doesn't exist.
    #[error("Gallery ID '{gallery_id}' not found and cannot be deleted")]
    GalleryNotFound { gallery_id: GalleryId },
    /// Emitted when a new gallery's ID already exists.
    #[error("Gallery ID '{gallery_id}' already exists and cannot be added again")]
    GalleryAlreadyExists { gallery_id: GalleryId },
}

/// The types of messages that scheduler module can take.
#[derive(Debug)]
pub enum SchedulerMessage {
    NewGallery(NewGalleryMessage),
    DeleteGallery(DeleteGalleryMessage),
    UpdateGallery(UpdateGalleryMessage)
}

/// Message for adding a new gallery to the scheduler.
pub type NewGalleryMessage = ModuleMessageWithReturn<GalleryInitializationState, Result<(), SchedulerError>>;

/// Message for deleting a gallery in the scheduler.
pub type DeleteGalleryMessage = ModuleMessageWithReturn<GalleryId, Result<(), SchedulerError>>;

/// Message for editing a gallery in the scheduler.
pub type UpdateGalleryMessage = ModuleMessageWithReturn<GalleryInitializationState, Result<(), SchedulerError>>;