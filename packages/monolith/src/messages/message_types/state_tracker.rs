use crate::galleries::{domain_types::GalleryId, pipeline_states::{GalleryPipelineStateTypes, GalleryPipelineStates, GallerySearchScrapingState}};
use super::ModuleMessageWithReturn;
use redis::RedisError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Possible errors emitted from the state tracker.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum StateTrackerError {
    #[error("Gallery already exists")]
    GalleryAlreadyExists,
    #[error("Gallery does not exist")]
    GalleryDoesntExist,
    #[error("Gallery has the wrong state")]
    GalleryHasWrongState,
    #[error("{0}")]
    Other(String)
}

// Map a Redis error into the Other variant.
impl From<RedisError> for StateTrackerError {
    fn from(err: RedisError) -> Self {
        StateTrackerError::Other(err.to_string())
    }
}

// Map a Redis error into the Other variant.
impl From<serde_json::Error> for StateTrackerError {
    fn from(err: serde_json::Error) -> Self {
        StateTrackerError::Other(err.to_string())
    }
}

/// The types of messages that the state tracker module can take.
#[derive(Debug)]
pub enum StateTrackerMessage {
    /// Add a gallery to the state.
    /// 
    /// Returns an `Err` if the gallery is already in state.
    AddGallery(AddGalleryMessage),
    /// Check if a gallery is in state.
    /// 
    /// Returns an `Err` if it isn't (not intuitive, but allows one to use the returned `StateTrackerError`)
    CheckGalleryDoesntExist(CheckGalleryDoesntExistMessage),
    /// Take the gallery's state (leaving the stored state as `None`).
    /// 
    /// Returns an `Err` if the gallery doesn't exist, its state has already been taken, or the requested state type doesn't match the stored state.
    GetGalleryState(GetGalleryStateMessage),
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
pub type AddGalleryMessage = ModuleMessageWithReturn<(GalleryId, GalleryPipelineStates), Result<(), StateTrackerError>>;

/// Message for checking a gallery's existence in the state.
pub type CheckGalleryDoesntExistMessage = ModuleMessageWithReturn<GalleryId, Result<(), StateTrackerError>>;

/// Message for checking a gallery's state.
pub type CheckGalleryStateMessage = ModuleMessageWithReturn<(GalleryId, GalleryPipelineStateTypes), Result<(), StateTrackerError>>;

/// Message for taking a gallery's state, leaving it set as `None`.
pub type GetGalleryStateMessage = ModuleMessageWithReturn<(GalleryId, GalleryPipelineStateTypes), Result<GalleryPipelineStates, StateTrackerError>>;

/// Message for updating and overwriting a gallery's state. 
pub type UpdateGalleryStateMessage = ModuleMessageWithReturn<(GalleryId, GalleryPipelineStates), Result<(), StateTrackerError>>;

/// Message for removing a gallery from the state.
pub type RemoveGalleryMessage = ModuleMessageWithReturn<GalleryId, Result<(), StateTrackerError>>;


