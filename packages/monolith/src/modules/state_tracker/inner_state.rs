use std::{collections::HashMap, future::Future};
use crate::galleries::{domain_types::GalleryId, pipeline_states::{GalleryPipelineStateTypes, GalleryPipelineStates}};

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
    pub fn add_gallery(&mut self, gallery_id: GalleryId, gallery_state: GalleryPipelineStates) -> Result<(), ()> {
        if self.states.contains_key(&gallery_id) {
            return Err(());
        }
        let state_type = gallery_state.state_type();
        self.states.insert(gallery_id, (state_type, Some(gallery_state)));
        Ok(())
    }

    /// Check if a gallery exists.
    pub fn check_gallery(&mut self, gallery_id: GalleryId) -> bool {
        self.states.contains_key(&gallery_id)
    }

    /// Check if a gallery matches the given state type.
    /// 
    /// Returns an `Err` if the gallery doesn't exist.
    pub fn check_gallery_state(&mut self, gallery_id: GalleryId, gallery_state: GalleryPipelineStateTypes) -> Result<bool, ()> {
        match self.states.get(&gallery_id) {
            Some((_, stored_state)) => Ok(matches!(stored_state, gallery_state)),
            None => Err(())
        }
    }

    /// Take the gallery's state, leaving it set as `None`.
    /// 
    /// Returns an `Err` if the gallery doesn't exist, the state has already been taken or the state doesn't match the requested type.
    pub fn take_gallery_state(&mut self, gallery_id: GalleryId, requested_state_type: GalleryPipelineStateTypes) -> Result<GalleryPipelineStates, ()> {
        match self.states.get_mut(&gallery_id) {
            Some((state_type, takeable_state)) => {
                if matches!(state_type, requested_state_type) {
                    return takeable_state.take().ok_or(());
                }
                Err(())
            },
            None => Err(())
        }
    }

    /// Update a gallery's state.
    /// 
    /// Returns an `Err` if the gallery doesn't exist, or the state isn't taken.
    pub fn update_gallery_state(&mut self, gallery_id: GalleryId, updated_state: GalleryPipelineStates) -> Result<(), ()> {
        match self.states.get_mut(&gallery_id) {
            Some((state_type, internal_state)) => {
                if internal_state.is_some() {
                    return Err(());
                };
                *state_type = updated_state.state_type();
                *internal_state = Some(updated_state);
            },
            None => return Err(())
        }
        Ok(())
    }

    /// Remove a gallery from the state.
    /// 
    /// Returns an `Err` if the gallery doesn't exist.
    pub fn remove_gallery(&mut self, gallery_id: GalleryId) -> Result<(), ()> {
        match self.states.remove(&gallery_id) {
            Some(_) => Ok(()),
            None => Err(())
        }
    }
}