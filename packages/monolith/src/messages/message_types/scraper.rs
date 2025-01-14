use serde::{Serialize, Deserialize};
use crate::galleries::{domain_types::{GalleryId, ItemId, Marketplace, UnixUtcDateTime}, items::item_data::MarketplaceItemData, pipeline_states::GalleryScrapingState};
use thiserror::Error;

/// Possible errors emitted from the scraper.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum ScraperError {
    #[error("Gallery {gallery_id}: {error}")]
    StartScrapingGalleryError { gallery_id: GalleryId, error: String },
    #[error("Gallery {gallery_id} ({marketplace}): {error}")]
    IngestScrapedSearchError { gallery_id: GalleryId, marketplace: Marketplace, error: String },
    #[error("Gallery {gallery_id} ({marketplace}): {error}")]
    IngestScrapedItemsError { gallery_id: GalleryId, marketplace: Marketplace, error: String }
}

/// The types of messages that the scraper module can take.
#[derive(Debug)]
pub enum ScraperMessage {
    /// This is the trigger for starting a new scraping job for a gallery.
    StartScrapingGallery(StartScrapingGallery),

    /// This is passed from an endpoint for the search scraper, consisting of scraped item IDs.
    /// 
    /// Each item should then either be fetched from storage, or scraped further.
    IngestScrapedSearch(IngestScrapedSearch),

    /// This is passed from an endpoint for the individual scraper, consisting of newly scraped item data.
    /// 
    /// This signifies that the data should be processed. 
    /// 
    /// If this is the last marketplace to be scraped under its gallery, it will also trigger the sending of 
    /// all the gallery's scraped items to the next stage.
    IngestScrapedItems(IngestScrapedItems)
}

/// Message to start the search scrape of a gallery.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StartScrapingGallery {
    pub gallery: GalleryScrapingState
}

/// Message to ingest the scraped search of a gallery's marketplace.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IngestScrapedSearch {
    pub gallery_id: GalleryId,
    pub marketplace: Marketplace,
    pub scraped_item_ids: Vec<ItemId>,
    pub updated_up_to: UnixUtcDateTime
}

/// Message to ingest the scraped items of a gallery's marketplace.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IngestScrapedItems {
    pub gallery_id: GalleryId,
    pub marketplace: Marketplace,
    pub scraped_items: Vec<MarketplaceItemData>
}