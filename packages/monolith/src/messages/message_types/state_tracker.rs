use crate::{galleries::domain_types::GalleryId, modules::state_tracker::gallery_state::GalleryState};
use super::ModuleMessageWithReturn;

/// The types of messages that the state tracker module can take.
#[derive(Debug)]
pub enum StateTrackerMessage {
    /// Add a gallery to the state.
    AddNewGallery(AddNewGalleryMessage),
    /// Take the gallery's state (leaving the stored state as `None`).
    /// 
    /// Returns an `Err` if the gallery doesn't exist, or its state has already been taken.
    /// 
    /// **NOTE**: The onus is on the user to put back the state after they have taken it.
    TakeGalleryState(TakeGalleryStateMessage),
    /// Put back the gallery's state, after one has taken it.
    /// 
    /// Returns an `Err` if the gallery doesn't exist, or state hasn't been taken.
    PutGalleryState(PutGalleryStateMessage),
    /// Remove a gallery from the state.
    /// 
    /// Returns an `Err` if the gallery doesn't exist.
    RemoveGallery(RemoveGalleryMessage)
}

/// Message for adding a new gallery to the state.
pub type AddNewGalleryMessage = ModuleMessageWithReturn<GalleryId, Result<(), ()>>;

/// Message for fetching a gallery's state.
pub type TakeGalleryStateMessage = ModuleMessageWithReturn<GalleryId, Result<GalleryState, ()>>;

/// Message for updating a gallery's state.
pub type PutGalleryStateMessage = ModuleMessageWithReturn<(GalleryId, GalleryState), Result<(), ()>>;

/// Message for removing a gallery from the state.
pub type RemoveGalleryMessage = ModuleMessageWithReturn<GalleryId, Result<(), ()>>;


