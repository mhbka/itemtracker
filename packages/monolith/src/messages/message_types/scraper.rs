use serde::{Serialize, Deserialize};
use crate::galleries::{domain_types::{GalleryId, ItemId, Marketplace, UnixUtcDateTime}, items::item_data::MarketplaceItemData, pipeline_states::GalleryScrapingState};
use super::ModuleMessage;
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

/// The types of messages the scraper module can take.
#[derive(Debug)]
pub enum ScraperMessage {
    /// This is the trigger for starting a new scraping job for a gallery.
    StartScrapingGallery(StartScrapingGalleryMessage),

    /// This is passed from an endpoint for the search scraper, consisting of scraped item IDs.
    /// 
    /// Each item should then either be fetched from storage, or scraped further.
    IngestScrapedSearch(IngestScrapedSearchMessage),

    /// This is passed from an endpoint for the individual scraper, consisting of newly scraped item data.
    /// 
    /// This signifies that the data should be processed. 
    /// 
    /// If this is the last marketplace to be scraped under its gallery, it will also trigger the sending of 
    /// all the gallery's scraped items to the next stage.
    IngestScrapedItems(IngestScrapedItemsMessage)
}

/// Message for starting a new scraping task for the gallery.
pub type StartScrapingGalleryMessage = ModuleMessage<StartScrapingGallery>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StartScrapingGallery {
    pub gallery: GalleryScrapingState
}

/// Message for sending scraped item IDs back to the scraper module, to be cache-fetched/individually scraped.
pub type IngestScrapedSearchMessage = ModuleMessage<IngestScrapedSearch>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IngestScrapedSearch {
    pub gallery_id: GalleryId,
    pub marketplace: Marketplace,
    pub scraped_item_ids: Vec<ItemId>,
    pub updated_up_to: UnixUtcDateTime
}

/// Message for sending freshly scraped items back to the scraper module, to be processed and sent to the next stage.
pub type IngestScrapedItemsMessage = ModuleMessage<IngestScrapedItems>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IngestScrapedItems {
    pub gallery_id: GalleryId,
    pub marketplace: Marketplace,
    pub scraped_items: Vec<MarketplaceItemData>
}