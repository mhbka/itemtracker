use serde::{Serialize, Deserialize};
use crate::{domain::{domain_types::GalleryId, pipeline_states::GalleryItemScrapingState}};
use thiserror::Error;

/// Possible errors emitted from the scraper.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum ItemScraperError {
    #[error("All items for all marketplaces for gallery {gallery_id} failed to scrape")]
    TotalScrapeFailure { gallery_id: GalleryId },
    #[error("Encountered an different error for gallery {gallery_id}: {message}")]
    Other { gallery_id: GalleryId, message: String }
}
