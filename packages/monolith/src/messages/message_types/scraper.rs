use serde::{Serialize, Deserialize};
use crate::galleries::{domain_types::GalleryId, pipeline_states::GalleryScrapingState};
use thiserror::Error;

/// Possible errors emitted from the scraper.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum ScraperError {
    #[error("Gallery {gallery_id}: {error}")]
    StartScrapingGalleryError { gallery_id: GalleryId, error: String }
}

/// The types of messages that the scraper module can take.
#[derive(Debug)]
pub enum ScraperMessage {
    /// This is the trigger for starting a new scraping job for a gallery.
    StartScrapingGallery { gallery: GalleryScrapingState }
}