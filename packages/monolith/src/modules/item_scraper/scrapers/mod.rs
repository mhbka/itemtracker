use std::collections::HashMap;

use futures::future::join_all;
use mercari::MercariItemScraper;
use crate::{config::ItemScraperConfig, galleries::{domain_types::{ItemId, Marketplace}, items::item_data::MarketplaceItemData}};

mod mercari;

/// This scraper is in charge of scraping detailed data for each item ID.
pub(super) struct ItemScraper { 
    config: ItemScraperConfig,
    mercari_scraper: MercariItemScraper
}

impl ItemScraper {
    /// Instantiate a `IndividualScraper`.
    pub fn new(config: &ItemScraperConfig) -> Self {
        Self {
            config: config.clone(),
            mercari_scraper: MercariItemScraper::new()
        }
    }

    /// Attempt to scrape a list of item IDs,
    /// returning a list with (in order) the item's data, or an `Err` if the item's scrape wasn't successful.
    /// 
    /// Returns the list with a single `Err` if the *dpop* key generation was unsuccessful (should never happen).
    pub async fn scrape_items(
        &self, 
        valid_scraped_search_ids: HashMap<Marketplace, Vec<ItemId>>
    ) -> HashMap<Marketplace, Vec<Result<MarketplaceItemData, String>>> {
        let results = join_all(
            valid_scraped_search_ids
                .into_iter()
                .map(|(marketplace, item_ids)| async {
                    let item_results = match marketplace {
                        Marketplace::Mercari => self.mercari_scraper.request(item_ids).await
                    };
                    (marketplace, item_results)
                })
            ).await;
        results
            .into_iter()
            .collect()
    }
}