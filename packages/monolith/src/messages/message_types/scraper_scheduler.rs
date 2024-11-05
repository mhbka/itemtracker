use thiserror::Error;
use serde::{Deserialize, Serialize};
use crate::galleries::{domain_types::GalleryId, scraping_pipeline::GalleryInitializationState};
use super::ModuleMessageWithReturn;

/// Possible errors emitted from the scraper scheduler.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum SchedulerError {
    /// Emitted when a deleted gallery's ID doesn't exist.
    #[error("Gallery ID '{gallery_id}' not found and cannot be deleted")]
    GalleryNotFound{ gallery_id: GalleryId },
    /// Emitted when a new gallery's ID already exists.
    #[error("Gallery ID '{gallery_id}' already exists and cannot be added again")]
    GalleryAlreadyExists{ gallery_id: GalleryId },
}

/// Possible messages that a scheduler can take.
#[derive(Debug)]
pub enum SchedulerMessage {
    NewGallery(NewGalleryMessage),
    DeleteGallery(DeleteGalleryMessage),
    EditGallery(EditGalleryMessage)
}

/// Message for adding a new gallery to the scheduler.
type NewGalleryMessage = ModuleMessageWithReturn<NewGallery, Result<(), SchedulerError>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewGallery {
    pub gallery: GalleryInitializationState
}

/// Message for deleting a gallery in the scheduler.
type DeleteGalleryMessage = ModuleMessageWithReturn<DeleteGallery, Result<(), SchedulerError>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeleteGallery {
    pub gallery_id: GalleryId
}

/// Message for editing a gallery in the scheduler.
type EditGalleryMessage = ModuleMessageWithReturn<EditGallery, Result<(), SchedulerError>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EditGallery {
    pub gallery: GalleryInitializationState
}

