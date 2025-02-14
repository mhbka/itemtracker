use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::galleries::{domain_types::GalleryId, pipeline_states::{GalleryClassifierState, GalleryFinalState, GalleryItemAnalysisState, GalleryItemScrapingState, GalleryPipelineStates, GallerySchedulerState, GallerySearchScrapingState}};

/// A mirror of the actual states, but allowing for takeability.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TakeableGalleryStates {
    Initialization(Option<GallerySchedulerState>),
    SearchScraping(Option<GallerySearchScrapingState>),
    ItemScraping(Option<GalleryItemScrapingState>),
    ItemAnalysis(Option<GalleryItemAnalysisState>),
    Classification(Option<GalleryClassifierState>),
    Final(Option<GalleryFinalState>)
}

impl TakeableGalleryStates {
    /// Map from `TakeableGalleryStates` to `Option<GalleryPipelineStates>`.
    fn map_from(self) -> Option<GalleryPipelineStates> {
        match self {
            TakeableGalleryStates::Initialization(state) => if let Some(state) = state { return Some(GalleryPipelineStates::Initialization(state)); },
            TakeableGalleryStates::SearchScraping(state) => if let Some(state) = state { return Some(GalleryPipelineStates::SearchScraping(state)); },
            TakeableGalleryStates::ItemScraping(state) => if let Some(state) = state { return Some(GalleryPipelineStates::ItemScraping(state)); },
            TakeableGalleryStates::ItemAnalysis(state) => if let Some(state) = state { return Some(GalleryPipelineStates::ItemAnalysis(state)); },
            TakeableGalleryStates::Classification(state) => if let Some(state) = state { return Some(GalleryPipelineStates::Classification(state)); },
            TakeableGalleryStates::Final(state) => if let Some(state) = state { return Some(GalleryPipelineStates::Final(state)); },
        }
        None
    }

    fn map_into(actual_state: GalleryPipelineStates) -> Self {
        match actual_state {
            GalleryPipelineStates::Initialization(state) => TakeableGalleryStates::Initialization(Some(state)),
            GalleryPipelineStates::SearchScraping(state) => TakeableGalleryStates::SearchScraping(Some(state)),
            GalleryPipelineStates::ItemScraping(state) => TakeableGalleryStates::ItemScraping(Some(state)),
            GalleryPipelineStates::ItemAnalysis(state) => TakeableGalleryStates::ItemAnalysis(Some(state)),
            GalleryPipelineStates::Classification(state) => TakeableGalleryStates::Classification(Some(state)),
            GalleryPipelineStates::Final(state) => TakeableGalleryStates::Final(Some(state)),
        }
    }
}

/// Stores + manages the actual state of galleries.
pub struct InnerState {
    states: HashMap<GalleryId, TakeableGalleryStates>
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
        let new_state = TakeableGalleryStates::map_into(gallery_state);
        self.states.insert(gallery_id, new_state);
        Ok(())
    }

    /// Check if a gallery exists.
    pub fn check_gallery(&mut self, gallery_id: GalleryId) -> bool {
        self.states.contains_key(&gallery_id)
    }

    /// Take the gallery's state, leaving it set as `None`.
    /// 
    /// Returns an `Err` if the gallery doesn't exist or the state has already been taken.
    pub fn take_gallery_state(&mut self, gallery_id: &GalleryId) -> Result<GalleryPipelineStates, ()> {
        match self.states.remove(gallery_id) {
            Some(state) => state.map_from().ok_or(()),
            None => Err(())
        }
    }

    /// Put back the gallery's state after taking it.
    /// 
    /// Returns an `Err` if the gallery doesn't exist, the state isn't taken, or the state given is not the correct kind.
    pub fn put_gallery_state(&mut self, gallery_id: GalleryId, gallery_state: GalleryPipelineStates) -> Result<(), ()> {
        match self.states.get_mut(&gallery_id) {
            Some(internal_state) => match (&*internal_state, gallery_state) {
                (TakeableGalleryStates::Initialization(_), GalleryPipelineStates::Initialization(state)) => 
                    *internal_state = TakeableGalleryStates::Initialization(Some(state)),
                (TakeableGalleryStates::SearchScraping(_), GalleryPipelineStates::SearchScraping(state)) => 
                    *internal_state = TakeableGalleryStates::SearchScraping(Some(state)),
                (TakeableGalleryStates::ItemScraping(_), GalleryPipelineStates::ItemScraping(state)) => 
                    *internal_state = TakeableGalleryStates::ItemScraping(Some(state)),
                (TakeableGalleryStates::ItemAnalysis(_), GalleryPipelineStates::ItemAnalysis(state)) => 
                    *internal_state = TakeableGalleryStates::ItemAnalysis(Some(state)),
                (TakeableGalleryStates::Classification(_), GalleryPipelineStates::Classification(state)) => 
                    *internal_state = TakeableGalleryStates::Classification(Some(state)),
                (TakeableGalleryStates::Final(_), GalleryPipelineStates::Final(state)) => 
                    *internal_state = TakeableGalleryStates::Final(Some(state)),
                _ => return Err(()),
            },
            None => return Err(())
        }
        Ok(())
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