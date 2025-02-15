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
    /// Returns an `Err` if the gallery doesn't exist, its state has already been taken, or the requested state type doesn't match the stored state.
    TakeGalleryState(TakeGalleryStateMessage),
    /// Update a gallery's state, overwriting its old state.
    /// 
    /// Returns an `Err` if the gallery doesn't exist, or its state has not been taken.
    UpdateGalleryState(UpdateGalleryStateMessage),
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
pub type TakeGalleryStateMessage = ModuleMessageWithReturn<(GalleryId, GalleryPipelineStateTypes), Result<GalleryPipelineStates, ()>>;

/// Message for updating and overwriting a gallery's state. 
pub type UpdateGalleryStateMessage = ModuleMessageWithReturn<(GalleryId, GalleryPipelineStates), Result<(), ()>>;

/// Message for removing a gallery from the state.
pub type RemoveGalleryMessage = ModuleMessageWithReturn<GalleryId, Result<(), ()>>;


