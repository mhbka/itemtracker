use std::collections::HashMap;
use crate::{config::ItemScraperConfig, domain::{domain_types::Marketplace, item_data::MarketplaceItemData, pipeline_states::{GalleryItemAnalysisState, GalleryItemScrapingState}}};
use error::ItemScraperError;
use scrapers::Scraper;

pub mod error;
mod scrapers;

/// Handles scraping of item data based on item IDs.
#[derive(Clone)]
pub struct ItemScraper {
    scraper: Scraper
}

impl ItemScraper {
    /// Initialize the scraper.
    pub fn new(config: &ItemScraperConfig) -> Self {
        Self {
            scraper: Scraper::new(config)
        }
    }

    pub async fn scrape(&mut self, gallery_state: GalleryItemScrapingState) -> Result<GalleryItemAnalysisState, ItemScraperError> {
        let scrape_results = self.scraper
            .scrape_items(&gallery_state)
            .await;

        match scrape_results
            .iter()
            .all(|(_, result)| {
                result.len() > 0 && // we allow empty results, as long as they aren't all errors
                result.iter().all(|res| res.is_err())
            })
            {   
                // if all items are errors, remove gallery from state and return an Err
                true => return Err(ItemScraperError::TotalScrapeFailure { gallery_id: gallery_state.gallery_id }),
                false => {
                    let new_state = self.to_next_state(
                        scrape_results, 
                        gallery_state
                    );
                    return Ok(new_state);
                }
            }
    }

    /// Process the gallery's state into the next state.
    fn to_next_state(
        &self,
        scraped_items: HashMap<Marketplace, Vec<Result<MarketplaceItemData, String>>>,
        gallery_state: GalleryItemScrapingState,
    ) -> GalleryItemAnalysisState {
        let valid_items = scraped_items
            .into_iter()
            .map(|(marketplace, results)| {
                // NOTE: currently we just filter out errors; in the future, do we want to note down errors as well?
                let valid_items = results
                    .into_iter()
                    .filter_map(|res| res.ok())
                    .collect();
                (marketplace, valid_items)
            })
            .collect();

        GalleryItemAnalysisState {
            gallery_id: gallery_state.gallery_id,
            items: valid_items,
            marketplace_updated_datetimes: gallery_state.marketplace_updated_datetimes,
            failed_marketplace_reasons: gallery_state.failed_marketplace_reasons,
            evaluation_criteria: gallery_state.evaluation_criteria,
        }
    }
}