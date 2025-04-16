use serde::{Serialize, Deserialize};
use crate::domain::domain_types::GalleryId;
use thiserror::Error;

/// Possible errors emitted from the scraper.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum SearchScraperError {
    #[error("All marketplaces for gallery {gallery_id} failed to scrape")]
    TotalScrapeFailure { gallery_id: GalleryId },
    #[error("Encountered a different error for gallery {gallery_id}: {message}")]
    Other { gallery_id: GalleryId, message: String }
}