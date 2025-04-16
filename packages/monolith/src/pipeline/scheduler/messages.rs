use tokio::sync::oneshot::{channel, Receiver, Sender};
use crate::domain::{domain_types::GalleryId, pipeline_states::GallerySchedulerState};
use super::error::SchedulerError;

/// Types of messages the scheduler can take.
/// 
/// ### Use
/// Initialize a message + oneshot receiver using one of the initializer functions,
/// and pass the message to the scheduler's sender;
/// then, wait for the response using the oneshot receiver.
pub enum SchedulerMessage {
    AddGallery(AddGalleryMessage),
    DeleteGallery(DeleteGalleryMessage),
    UpdateGallery(UpdateGalleryMessage)
}

impl SchedulerMessage {
    pub fn add_gallery(gallery_state: GallerySchedulerState) -> (Self, Receiver<Result<(), SchedulerError>>) {
        let (sender, receiver) = channel();
        let msg = Self::AddGallery((gallery_state, sender));
        (msg, receiver)
    }

    pub fn delete_gallery(gallery_id: GalleryId) -> (Self, Receiver<Result<(), SchedulerError>>) {
        let (sender, receiver) = channel();
        let msg = Self::DeleteGallery((gallery_id, sender));
        (msg, receiver)
    }

    pub fn update_gallery(gallery_state: GallerySchedulerState) -> (Self, Receiver<Result<(), SchedulerError>>) {
        let (sender, receiver) = channel();
        let msg = Self::UpdateGallery((gallery_state, sender));
        (msg, receiver)
    }
}

pub type AddGalleryMessage = (GallerySchedulerState, Sender<Result<(), SchedulerError>>);

pub type DeleteGalleryMessage = (GalleryId, Sender<Result<(), SchedulerError>>);

pub type UpdateGalleryMessage = (GallerySchedulerState, Sender<Result<(), SchedulerError>>);