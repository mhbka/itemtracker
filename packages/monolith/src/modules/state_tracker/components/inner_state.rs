use std::collections::HashMap;

use crate::galleries::{domain_types::GalleryId, pipeline_states::{GallerySearchScrapingState, GalleryPipelineStates}};

/// Stores + manages the actual state of galleries.
pub struct InnerState {
    states: HashMap<GalleryId, Option<GalleryPipelineStates>>
}

impl InnerState {
    /// Instantiate the states.
    /// 
    /// TODO: Persist and get from a KV store like Redis later on?
    pub fn init() -> Self {
        Self { states: HashMap::new() }
    }

    /// Add a new gallery to the state.
    /// 
    /// Returns an `Err` if the gallery is already in state.
    pub fn add_gallery(&mut self, gallery_id: GalleryId, state: GallerySearchScrapingState) -> Result<(), ()> {
        if self.states.contains_key(&gallery_id) {
            return Err(());
        }
        let new_state = GalleryPipelineStates::SearchScraping(state);
        self.states.insert(gallery_id, Some(new_state));
        Ok(())
    }

    /// Take the gallery's state.
    /// 
    /// Returns an `Err` if the gallery doesn't exist or the state has already been taken.
    pub fn take_gallery_state(&mut self, gallery_id: &GalleryId) -> Result<GalleryPipelineStates, ()> {
        match self.states.remove(gallery_id) {
            Some(state) => state.ok_or(()),
            None => Err(())
        }
    }

    /// Put back the gallery's state.
    /// 
    /// Returns an `Err` if the gallery doesn't exist, or the state isn't taken.
    pub fn put_gallery_state(&mut self, gallery_id: GalleryId, gallery_state: GalleryPipelineStates) -> Result<(), ()> {
        match self.states.contains_key(&gallery_id) {
            false => {
                self.states.insert(gallery_id, Some(gallery_state));
                Ok(())
            },
            true => Err(())
        }
    }

    /// Remove a gallery from the state.
    /// 
    /// Returns an `Err` if the gallery doesn't exist.
    pub fn remove_gallery(&mut self, gallery_id: &GalleryId) -> Result<(), ()> {
        match self.states.remove(gallery_id) {
            Some(_) => Ok(()),
            None => Err(())
        }
    }
}