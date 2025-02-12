use std::collections::HashMap;

use futures::future::join_all;
use mercari::MercariSearchScraper;
use crate::{config::SearchScraperConfig, galleries::{domain_types::{ItemId, Marketplace}, items::item_data::MarketplaceItemData, pipeline_states::GallerySearchScrapingState}};

mod mercari;

/// This scraper is in charge of using item IDs to scrape detailed data for each item.
pub(super) struct SearchScraper {
    config: SearchScraperConfig,
    mercari_scraper: MercariSearchScraper
}

impl SearchScraper {
    /// Instantiate a `SearchScraper`.
    pub fn new(config: &SearchScraperConfig) -> Self {
        SearchScraper {
            config: config.clone(),
            mercari_scraper: MercariSearchScraper::new()
        }
    }

    /// Attempt to scrape item IDs according to a search criteria.
    /// 
    /// Returns an `Err` for whichever marketplaces had errors while scraping.
    pub async fn scrape_search(&mut self, gallery: &GallerySearchScrapingState) -> HashMap<Marketplace, Result<Vec<ItemId>, String>> {
        let results = join_all(
            gallery.marketplace_previous_scraped_datetimes
                .clone()
                .into_iter()
                .map(|(marketplace, previous_scraped_item_datetime)| async {
                    let item_ids = match marketplace {
                        Marketplace::Mercari => self.mercari_scraper
                            .request(&gallery.search_criteria, previous_scraped_item_datetime)
                            .await
                    };
                    (marketplace, item_ids)
                })
            ).await;
        results
            .into_iter()
            .collect()
    }
}