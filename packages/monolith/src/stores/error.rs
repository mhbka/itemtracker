use diesel_async::pooled_connection::deadpool::PoolError;
use thiserror::Error;
use uuid::Uuid;

/// Errors that may arise while accessing a store.
#[derive(Error, Debug)]
pub enum StoreError {
    #[error("No gallery with ID {gallery_id} found")]
    NotFound { gallery_id: Uuid },
    #[error("Error from the pool: {0}")]
    Pool(#[from] PoolError),
    #[error("Error from the database: {0}")]
    Database(#[from] diesel::result::Error),
    #[error("Error does not fit into category: {message}")]
    Other { message: String }
}

/// Alias for a result whose error is `StoreError`.
pub type StoreResult<T> = Result<T, StoreError>;

