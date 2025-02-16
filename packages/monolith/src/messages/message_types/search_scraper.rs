use serde::{Serialize, Deserialize};
use crate::galleries::{domain_types::GalleryId, pipeline_states::GallerySearchScrapingState};
use thiserror::Error;

use super::state_tracker::StateTrackerError;

/// Possible errors emitted from the scraper.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum SearchScraperError {
    #[error("All marketplaces for gallery {gallery_id} failed to scrape")]
    TotalScrapeFailure { gallery_id: GalleryId },
    #[error("Error from state tracker for gallery {gallery_id}: {err}")]
    StateErr { gallery_id: GalleryId, err: StateTrackerError },
    #[error("Encountered a different error for gallery {gallery_id}: {message}")]
    Other { gallery_id: GalleryId, message: String }
}

/// The types of messages that the scraper module can take.
#[derive(Debug)]
pub enum SearchScraperMessage {
    /// This is for starting an item scrape, using the ID of a gallery in state.
    /// If the gallery ID is not in state, an error is logged and nothing happens.
    ScrapeSearch { gallery_id: GalleryId },
    /// This is for starting a search scrape, using a search-scraped gallery's data.
    ScrapeSearchNew { gallery: GallerySearchScrapingState }
}