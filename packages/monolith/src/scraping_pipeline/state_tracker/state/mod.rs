use internal::InternalState;
use redis::RedisState;
use crate::{config::state_tracker::StateTrackerConfig, domain::{domain_types::GalleryId, pipeline_states::{GalleryPipelineStateTypes, GalleryPipelineStates}}, messages::message_types::state_tracker::StateTrackerError};

mod internal;
mod redis;

/// The interface for the inner state of the state tracker.
pub(super) trait State {
    /// Add a gallery to the state.
    /// 
    /// Returns an `Err` if the gallery is already exists.
    async fn add_gallery(&mut self, gallery_id: GalleryId, gallery_state: GalleryPipelineStates) -> Result<(), StateTrackerError>;

    /// Verify that a gallery *doesn't* exist.
    /// 
    /// Returns an `Err` if it exists.
    async fn check_gallery_doesnt_exist(&mut self, gallery_id: GalleryId) -> Result<(), StateTrackerError>;

    /// Get the gallery's state.
    /// 
    /// Returns an `Err` if the gallery doesn't exist, or the state doesn't match the requested type.
    async fn get_gallery_state(&mut self, gallery_id: GalleryId, requested_state_type: GalleryPipelineStateTypes) -> Result<GalleryPipelineStates, StateTrackerError>;

    /// Update a gallery's state.
    /// 
    /// Returns an `Err` if the gallery doesn't exist.
    async fn update_gallery_state(&mut self, gallery_id: GalleryId, updated_state: GalleryPipelineStates) -> Result<(), StateTrackerError>;

    /// Remove a gallery from the state.
    /// 
    /// Returns an `Err` if the gallery doesn't exist.
    async fn remove_gallery(&mut self, gallery_id: GalleryId) -> Result<(), StateTrackerError>;
}

/// The inner state of the state tracker.
pub(super) enum InnerState {
    Internal(InternalState),
    Redis(RedisState)
}

impl InnerState {
    pub(super) async fn init(config: &StateTrackerConfig) -> Self {
        if config.use_redis {
            if let Ok(state) = RedisState::init(config).await {
                return Self::Redis(state);
            } else {
                tracing::warn!("Failed to connect to Redis for state tracker; falling back to internal state...");
            }
        }
        Self::Internal(InternalState::init())
    }
}

impl State for InnerState {
    async fn add_gallery(&mut self, gallery_id: GalleryId, gallery_state: GalleryPipelineStates) -> Result<(), StateTrackerError> {
        match self {
            InnerState::Internal(state) => state.add_gallery(gallery_id, gallery_state).await,
            InnerState::Redis(state) => state.add_gallery(gallery_id, gallery_state).await,
        }
    }

    async fn check_gallery_doesnt_exist(&mut self, gallery_id: GalleryId) -> Result<(), StateTrackerError> {
        match self {
            InnerState::Internal(state) => state.check_gallery_doesnt_exist(gallery_id).await,
            InnerState::Redis(state) => state.check_gallery_doesnt_exist(gallery_id).await,
        }
    }

    async fn get_gallery_state(&mut self, gallery_id: GalleryId, requested_state_type: GalleryPipelineStateTypes) -> Result<GalleryPipelineStates, StateTrackerError> {
        match self {
            InnerState::Internal(state) => state.get_gallery_state(gallery_id, requested_state_type).await,
            InnerState::Redis(state) => state.get_gallery_state(gallery_id, requested_state_type).await,
        }
    }

    async fn update_gallery_state(&mut self, gallery_id: GalleryId, updated_state: GalleryPipelineStates) -> Result<(), StateTrackerError> {
        match self {
            InnerState::Internal(state) => state.update_gallery_state(gallery_id, updated_state).await,
            InnerState::Redis(state) => state.update_gallery_state(gallery_id, updated_state).await,
        }
    }

    async fn remove_gallery(&mut self, gallery_id: GalleryId) -> Result<(), StateTrackerError> {
        match self {
            InnerState::Internal(state) => state.remove_gallery(gallery_id).await,
            InnerState::Redis(state) => state.remove_gallery(gallery_id).await,
        }
    }
}