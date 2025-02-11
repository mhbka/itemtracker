use std::sync::Arc;

use mercari::MercariSearchScraper;
use tokio::sync::Mutex;
use crate::{config::ScraperConfig, galleries::{domain_types::{GalleryId, Marketplace}, pipeline_states::GalleryScrapingState}};
use super::state_manager::GalleryStates;

mod mercari;

/// This scraper is in charge of using item IDs to scrape detailed data for each item.
pub(super) struct SearchScraper {
    config: ScraperConfig,
    mercari_scraper: MercariSearchScraper
}

impl SearchScraper {
    /// Instantiate a `SearchScraper`.
    pub fn new(config: &ScraperConfig) -> Self {
        SearchScraper {
            config: config.clone(),
            mercari_scraper: MercariSearchScraper::new()
        }
    }

    /// Schedules scraping jobs to scrape each marketplace's search within the gallery.
    pub async fn schedule_scrape_search(
        &mut self, 
        gallery: &GalleryScrapingState, 
        gallery_states: Arc<Mutex<GalleryStates>>
    ) {   
        for marketplace in &gallery.marketplaces {
            let scrape_result = match marketplace {
                Marketplace::Mercari => self.mercari_scraper.request(gallery).await,
            };
        }
    }
}