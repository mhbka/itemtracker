use crate::galleries::{domain_types::GalleryId, pipeline_states::{GalleryPipelineStateTypes, GalleryPipelineStates, GallerySearchScrapingState}};
use super::ModuleMessageWithReturn;

/// The types of messages that the state tracker module can take.
#[derive(Debug)]
pub enum StateTrackerMessage {
    /// Add a gallery to the state.
    /// 
    /// Returns an `Err` if the gallery is already in state.
    AddGallery(AddGalleryMessage),
    /// Check if a gallery is in state.
    CheckGallery(CheckGalleryMessage),
    /// Check the gallery's state.
    /// 
    /// Returns an `Err` if the gallery doesn't exist.
    CheckGalleryState(CheckGalleryStateMessage),
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
pub type AddGalleryMessage = ModuleMessageWithReturn<(GalleryId, GalleryPipelineStates), Result<(), ()>>;

/// Message for checking a gallery's existence in the state.
pub type CheckGalleryMessage = ModuleMessageWithReturn<GalleryId, bool>;

/// Message for checking a gallery's state.
pub type CheckGalleryStateMessage = ModuleMessageWithReturn<(GalleryId, GalleryPipelineStateTypes), Result<bool, ()>>;

/// Message for taking a gallery's state, leaving it set as `None`.
pub type TakeGalleryStateMessage = ModuleMessageWithReturn<GalleryId, Result<GalleryPipelineStates, ()>>;

/// Message for putting back a gallery's state after it has been taken.
pub type PutGalleryStateMessage = ModuleMessageWithReturn<(GalleryId, GalleryPipelineStates), Result<(), ()>>;

/// Message for updating and overwriting a gallery's state.
pub type UpdateGalleryStateMessage = ModuleMessageWithReturn<(GalleryId, GalleryPipelineStates), ()>;

/// Message for removing a gallery from the state.
pub type RemoveGalleryMessage = ModuleMessageWithReturn<GalleryId, Result<(), ()>>;


