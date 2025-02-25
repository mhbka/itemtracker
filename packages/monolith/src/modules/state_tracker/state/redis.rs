use std::error::Error;
use axum::async_trait;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client, JsonAsyncCommands};
use crate::{config::state_tracker::StateTrackerConfig, galleries::{domain_types::GalleryId, pipeline_states::{GalleryPipelineStateTypes, GalleryPipelineStates}}, messages::message_types::state_tracker::StateTrackerError};
use super::State;

/// The Redis-backed inner state of the state tracker. 
/// 
/// Allows for persisting of pipeline states.
pub struct RedisState {
    client: Client,
    connection: MultiplexedConnection
}

impl RedisState {
    /// Initialize the state.
    /// 
    /// Returns an `Err` if unable to connect to the Redis instance for some reason.
    pub async fn init(config: &StateTrackerConfig) -> Result<Self, ()> {
        let client = Client::open(config.redis_uri.as_str())
            .map_err(|err| {
                tracing::error!("Unable to connect to Redis client for state tracker: {err} (source: {:?})", err.source());
                ()
            })?;
        let connection = client.get_multiplexed_async_connection().await
            .map_err(|err| {
                tracing::error!("Unable to get Redis connection for state tracker: {err} (source: {:?})", err.source());
                ()
            })?;
        Ok(
            Self {
                client,
                connection
            }
        )
        
    }
}

impl State for RedisState {
    async fn add_gallery(&mut self, gallery_id: GalleryId, gallery_state: GalleryPipelineStates) -> Result<(), StateTrackerError> {
        let gallery_str = serde_json::to_string(&gallery_state)?;
        let res: Option<()> = self.connection
            .set_nx(gallery_id.as_str(), gallery_str)
            .await?;
        match res {
            Some(v) => return Ok(()),
            None => return Err(StateTrackerError::GalleryAlreadyExists)
        }
    }

    async fn check_gallery_doesnt_exist(&mut self, gallery_id: GalleryId) -> Result<(), StateTrackerError> {
        let res: bool = self.connection
            .exists(gallery_id.as_str())
            .await?;
        match res {
            true => Err(StateTrackerError::GalleryAlreadyExists),
            false => Ok(())
        }
    }

    async fn get_gallery_state(&mut self, gallery_id: GalleryId, requested_state_type: GalleryPipelineStateTypes) -> Result<GalleryPipelineStates, StateTrackerError> {
        let gallery_str: String = self.connection
            .get(gallery_id.as_str())
            .await?;
        let gallery: GalleryPipelineStates = serde_json::from_str(&gallery_str)?;
        match gallery.matches(&requested_state_type) {
            true => Ok(gallery),
            false => Err(StateTrackerError::GalleryHasWrongState)
        }
    }

    async fn update_gallery_state(&mut self, gallery_id: GalleryId, updated_state: GalleryPipelineStates) -> Result<(), StateTrackerError> {
        match self.connection
            .exists(gallery_id.as_str())
            .await?
        {
            true => {
                let gallery_str = serde_json::to_string(&updated_state)?;
                let _: () = self.connection
                    .set(gallery_id.as_str(), gallery_str)
                    .await?;
                Ok(())
            },
            false => Err(StateTrackerError::GalleryDoesntExist)
        }
    }

    async fn remove_gallery(&mut self, gallery_id: GalleryId) -> Result<(), StateTrackerError> {
        match self.connection
            .exists(gallery_id.as_str())
            .await?
        {
            true => {
                let _: () = self.connection
                    .del(gallery_id.as_str())
                    .await?;
                Ok(())
            },
            false => Err(StateTrackerError::GalleryDoesntExist)
        }
    }
}