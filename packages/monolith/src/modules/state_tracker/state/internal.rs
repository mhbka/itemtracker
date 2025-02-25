use std::collections::HashMap;
use axum::async_trait;

use crate::{galleries::{domain_types::GalleryId, pipeline_states::{GalleryPipelineStateTypes, GalleryPipelineStates}}, messages::message_types::state_tracker::StateTrackerError};
use super::State;

/// A hashmap-backed inner state for the state tracker.
/// 
/// Does not persist states anywhere.
pub struct InternalState {
    states: HashMap<GalleryId, GalleryPipelineStates>
}

impl InternalState {
    /// Initialize the internal state.
    pub fn init() -> Self {
        Self {
            states: HashMap::new()
        }
    }
}

impl State for InternalState {
    async fn add_gallery(&mut self, gallery_id: GalleryId, gallery_state: GalleryPipelineStates) -> Result<(), StateTrackerError> {
        if self.states.contains_key(&gallery_id) {
            return Err(StateTrackerError::GalleryAlreadyExists);
        }
        self.states.insert(gallery_id, gallery_state);
        Ok(())
    }

    async fn check_gallery_doesnt_exist(&mut self, gallery_id: GalleryId) -> Result<(), StateTrackerError> {
        if self.states.contains_key(&gallery_id) {
            return Err(StateTrackerError::GalleryAlreadyExists);
        }
        Ok(())
    }

    async fn get_gallery_state(&mut self, gallery_id: GalleryId, requested_state_type: GalleryPipelineStateTypes) -> Result<GalleryPipelineStates, StateTrackerError> {
        match self.states.get(&gallery_id) {
            Some(state) => {
                if state.matches(&requested_state_type) {
                    return Ok(state.clone());
                }
                Err(StateTrackerError::GalleryHasWrongState)
            },
            None => Err(StateTrackerError::GalleryDoesntExist)
        }
    }

    async fn update_gallery_state(&mut self, gallery_id: GalleryId, updated_state: GalleryPipelineStates) -> Result<(), StateTrackerError> {
        match self.states.get_mut(&gallery_id) {
            Some(state) => *state = updated_state,
            None => return Err(StateTrackerError::GalleryDoesntExist)
        }
        Ok(())
    }

    async fn remove_gallery(&mut self, gallery_id: GalleryId) -> Result<(), StateTrackerError> {
        match self.states.remove(&gallery_id) {
            Some(_) => Ok(()),
            None => Err(StateTrackerError::GalleryDoesntExist)
        }
    }
}