use crate::{domain::domain_types::GalleryId, stores::error::StoreError};
use thiserror::Error;

/// Possible errors emitted from storage.
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("{0}")]
    StoreErr(#[from] StoreError),
    #[error("Encountered a different error for gallery {gallery_id}: {message}")]
    Other { gallery_id: GalleryId, message: String }
}