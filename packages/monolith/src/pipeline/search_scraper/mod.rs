use std::collections::HashMap;
use crate::{config::SearchScraperConfig, domain::{domain_types::{GalleryId, ItemId, Marketplace, UnixUtcDateTime}, pipeline_states::{GalleryItemScrapingState, GallerySearchScrapingState}}};
use error::SearchScraperError;
use scrapers::Scraper;

pub mod error;
mod scrapers;

/// Handles scraping of searches for galleries.
#[derive(Clone)]
pub struct SearchScraper {
    scraper: Scraper
}

impl SearchScraper {
    /// Initialize the scraper.
    pub fn new(config: &SearchScraperConfig) -> Self {
        Self {
            scraper: Scraper::new(&config)
        }
    }

    /// Scrape the gallery.
    pub async fn scrape(&mut self, gallery_state: GallerySearchScrapingState) -> Result<GalleryItemScrapingState, SearchScraperError> {
        let scrape_results = self.scraper
            .scrape_search(&gallery_state)
            .await;
        let new_state = self.to_next_state(
            scrape_results, 
            gallery_state
        );
        Ok(new_state)
    }

    /// Process the gallery's state into the next state.
    fn to_next_state(
        &self,
        scrape_results: HashMap<Marketplace, Result<Vec<ItemId>, String>>,
        gallery_state: GallerySearchScrapingState,
    ) -> GalleryItemScrapingState {
        let cur_datetime = UnixUtcDateTime::now();
        let gallery_id = gallery_state.gallery_id;

        let marketplace_updated_datetimes = scrape_results
            .iter()
            .filter(|(_, result)| result.is_ok())
            .map(|(marketplace, _)| (*marketplace, cur_datetime))
            .collect();

        let failed_marketplace_reasons = scrape_results
            .iter()
            .map(|(m, r)| (m, r.clone()))
            .filter_map(|(marketplace, result)| result.err().map(|err| (*marketplace, err)))
            .collect();

        let valid_scraped_search_ids = scrape_results
            .into_iter()
            .filter_map(|(marketplace, result)| result.ok().map(|ids| (marketplace, ids)))
            .collect();

        tracing::info!(
            "Gallery {} collected the following:\n Item IDs: {:#?}\n Errors: {:#?}",
            gallery_id, valid_scraped_search_ids, failed_marketplace_reasons
        );
        
        GalleryItemScrapingState {
            gallery_id: gallery_state.gallery_id,
            item_ids: valid_scraped_search_ids,
            failed_marketplace_reasons,
            marketplace_updated_datetimes,
            evaluation_criteria: gallery_state.evaluation_criteria
        }
    }
}