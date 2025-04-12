use diesel_async::pooled_connection::deadpool::PoolError;
use thiserror::Error;
use uuid::Uuid;

/// Errors that may arise while accessing a store.
#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Error from the pool: {0}")]
    Pool(#[from] PoolError),
    #[error("Error from the database: {0}")]
    Database(#[from] diesel::result::Error),
    #[error("No gallery with ID {gallery_id} found")]
    NotFound { gallery_id: Uuid }
}