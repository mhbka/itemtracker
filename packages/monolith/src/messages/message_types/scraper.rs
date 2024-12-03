use serde::{Serialize, Deserialize};
use crate::galleries::{domain_types::{GalleryId, ItemId, Marketplace}, items::item_data::MarketplaceItemData, scraping_pipeline::GalleryScrapingState};
use super::ModuleMessageWithReturn;
use thiserror::Error;

/// Possible errors emitted from the scraper.
#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub enum ScraperError {
    /// Emitted when an error occurs while requesting a search scrape.
    #[error("Error while requesting search scrape for gallery {gallery_id}: {error}")]
    UnsuccessfulSearchScrapeRequest{ gallery_id: GalleryId, error: String },
    /// Emitted when an error occurs while requesting a search scrape.
    #[error("Error while requesting individual scrape for gallery {gallery_id} ({marketplace}): {error_str}")]
    UnsuccessfulIndivScrapeRequest{ gallery_id: GalleryId, marketplace: Marketplace, error_str: String },
}

/// The types of messages the scraper module can take.
#[derive(Debug)]
pub enum ScraperMessage {
    /// This is the trigger for starting a new scraping job for a gallery.
    StartScraping(StartScrapingJobMessage),

    /// This is passed from an endpoint for the search scraper, consisting of scraped item IDs.
    /// 
    /// Each item should then either be fetched from storage, or scraped further.
    ScrapeIndividualItems(ScrapeIndivItemsMessage),

    /// This is passed from an endpoint for the individual scraper, consisting of newly scraped item data.
    /// 
    /// This signifies that the data should be processed. 
    /// 
    /// If this is the last marketplace to be scraped under its gallery, it will also trigger the sending of 
    /// all the gallery's scraped items to the next stage.
    ProcessScrapedItems(ProcessScrapedItemsMessage)
}

/// Message for starting a new scraping task for the gallery.
pub type StartScrapingJobMessage = ModuleMessageWithReturn<StartScrapingJob, Result<(), ScraperError>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StartScrapingJob {
    pub gallery: GalleryScrapingState
}

/// Message for sending scraped item IDs back to the scraper module, to be cache-fetched/individually scraped.
pub type ScrapeIndivItemsMessage = ModuleMessageWithReturn<ScrapeIndivItems, Result<(), ScraperError>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScrapeIndivItems {
    pub gallery_id: GalleryId,
    pub marketplace: Marketplace,
    pub scraped_item_ids: Vec<ItemId>
}

/// Message for sending freshly scraped items back to the scraper module, to be processed and sent to the next stage.
pub type ProcessScrapedItemsMessage = ModuleMessageWithReturn<ProcessScrapedItems, Result<(), ScraperError>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProcessScrapedItems {
    pub gallery_id: GalleryId,
    pub marketplace: Marketplace,
    pub scraped_items: Vec<MarketplaceItemData>
}