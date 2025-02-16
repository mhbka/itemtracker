use std::{collections::HashMap, future::Future};
use crate::{galleries::{domain_types::GalleryId, pipeline_states::{GalleryPipelineStateTypes, GalleryPipelineStates}}, messages::message_types::state_tracker::StateTrackerError};

/// Stores + manages the state of galleries.
/// 
/// We use an `Option` for the actual state, to allow users to take and put back state data.
/// This allows for more efficient state updating, since a lot of data is common between states
/// and cloning it for updating would be inefficient.
/// 
/// However, this does necessitate that the user remember to put back state data, or update it with new state).
/// 
/// We keep the state's corresponding `GalleryPipelineStateTypes` 
/// to track its state type whenever it is taken (ie, `None`).
pub struct InnerState {
    states: HashMap<GalleryId, (GalleryPipelineStateTypes, Option<GalleryPipelineStates>)>
}

impl InnerState {
    /// Instantiate the states.
    /// 
    /// TODO: Persist and get from a KV store like Redis later on?
    pub fn init() -> Self {
        Self { states: HashMap::new() }
    }

    /// Add a gallery to the state.
    /// 
    /// Returns an `Err` if the gallery is already exists.
    pub fn add_gallery(&mut self, gallery_id: GalleryId, gallery_state: GalleryPipelineStates) -> Result<(), StateTrackerError> {
        if self.states.contains_key(&gallery_id) {
            return Err(StateTrackerError::GalleryAlreadyExists);
        }
        let state_type = gallery_state.state_type();
        self.states.insert(gallery_id, (state_type, Some(gallery_state)));
        Ok(())
    }

    /// Check that a gallery doesn't exist.
    /// 
    /// Returns an `Err` if it exists.
    pub fn check_gallery_doesnt_exist(&mut self, gallery_id: GalleryId) -> Result<(), StateTrackerError> {
        if self.states.contains_key(&gallery_id) {
            return Err(StateTrackerError::GalleryAlreadyExists);
        }
        Ok(())
    }

    /// Check if a gallery matches the given state type.
    /// 
    /// Returns an `Err` if the gallery doesn't exist.
    pub fn check_gallery_state(&mut self, gallery_id: GalleryId, gallery_state: GalleryPipelineStateTypes) -> Result<(), StateTrackerError> {
        match self.states.get(&gallery_id) {
            Some((_, stored_state)) => {
                match matches!(stored_state, gallery_state) {
                    true => Ok(()),
                    false => Err(StateTrackerError::GalleryHasWrongState)
                }   
            },
            None => Err(StateTrackerError::GalleryDoesntExist)
        }
    }

    /// Take the gallery's state, leaving it set as `None`.
    /// 
    /// Returns an `Err` if the gallery doesn't exist, the state has already been taken, or the state doesn't match the requested type.
    pub fn take_gallery_state(&mut self, gallery_id: GalleryId, requested_state_type: GalleryPipelineStateTypes) -> Result<GalleryPipelineStates, StateTrackerError> {
        match self.states.get_mut(&gallery_id) {
            Some((state_type, takeable_state)) => {
                if matches!(state_type, requested_state_type) {
                    return takeable_state.take().ok_or(StateTrackerError::GalleryStateIsTaken);
                }
                Err(StateTrackerError::GalleryHasWrongState)
            },
            None => Err(StateTrackerError::GalleryDoesntExist)
        }
    }

    /// Update a gallery's state.
    /// 
    /// Returns an `Err` if the gallery doesn't exist, or the state isn't taken.
    pub fn update_gallery_state(&mut self, gallery_id: GalleryId, updated_state: GalleryPipelineStates) -> Result<(), StateTrackerError> {
        match self.states.get_mut(&gallery_id) {
            Some((state_type, internal_state)) => {
                if internal_state.is_some() {
                    return Err(StateTrackerError::GalleryStateIsntTaken);
                };
                *state_type = updated_state.state_type();
                *internal_state = Some(updated_state);
            },
            None => return Err(StateTrackerError::GalleryDoesntExist)
        }
        Ok(())
    }

    /// Remove a gallery from the state.
    /// 
    /// Returns an `Err` if the gallery doesn't exist.
    pub fn remove_gallery(&mut self, gallery_id: GalleryId) -> Result<(), StateTrackerError> {
        match self.states.remove(&gallery_id) {
            Some(_) => Ok(()),
            None => Err(StateTrackerError::GalleryDoesntExist)
        }
    }
}