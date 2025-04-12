use thiserror::Error;
use serde::{Deserialize, Serialize};
use crate::{domain::{domain_types::GalleryId, pipeline_states::GallerySchedulerState}, messages::message_buses::MessageError};
use super::{state_tracker::StateTrackerError, ModuleMessageWithReturn};

/// Possible errors emitted from the scraper scheduler.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum SchedulerError {
    #[error("Gallery {gallery_id} not found and cannot be deleted")]
    GalleryNotFound { gallery_id: GalleryId },
    #[error("Gallery {gallery_id} already exists and cannot be added again")]
    GalleryAlreadyExists { gallery_id: GalleryId },
    #[error("Update for gallery {gallery_id} has the wrong gallery ID")]
    GalleryUpdateHasWrongId { gallery_id: GalleryId },
    #[error("Error from state tracker for gallery {gallery_id}: {err}")]
    StateErr { gallery_id: GalleryId, err: StateTrackerError },
    #[error("Error while sending a message for gallery {gallery_id}: {err}")]
    MessageErr { gallery_id: GalleryId, err: MessageError },
    #[error("Encountered a different error for gallery {gallery_id}: {message}")]
    Other { gallery_id: GalleryId, message: String },
    
}

/// The types of messages that scheduler module can take.
#[derive(Debug)]
pub enum SchedulerMessage {
    NewGallery(NewGalleryMessage),
    DeleteGallery(DeleteGalleryMessage),
    UpdateGallery(UpdateGalleryMessage)
}

/// Message for adding a new gallery to the scheduler.
pub type NewGalleryMessage = ModuleMessageWithReturn<GallerySchedulerState, Result<(), SchedulerError>>;

/// Message for deleting a gallery in the scheduler.
pub type DeleteGalleryMessage = ModuleMessageWithReturn<GalleryId, Result<(), SchedulerError>>;

/// Message for editing a gallery in the scheduler.
pub type UpdateGalleryMessage = ModuleMessageWithReturn<GallerySchedulerState, Result<(), SchedulerError>>;