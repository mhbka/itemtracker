use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::domain::domain_types::GalleryId;

/// Possible errors emitted from the item analysis module.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum ItemEmbedderError {
    #[error("Failed to embed any items for gallery {gallery_id}")]
    TotalEmbedFailure { gallery_id: GalleryId },
    #[error("Encountered an different error for gallery {gallery_id}: {message}")]
    Other { gallery_id: GalleryId, message: String }
}