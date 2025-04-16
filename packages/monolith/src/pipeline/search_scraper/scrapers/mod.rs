use std::collections::HashMap;
use futures::future::join_all;
use mercari::MercariSearchScraper;
use crate::{config::SearchScraperConfig, domain::{domain_types::{ItemId, Marketplace}, pipeline_states::GallerySearchScrapingState}};

mod mercari;

/// This scraper is in charge of using item IDs to scrape detailed data for each item.
#[derive(Clone)]
pub(super) struct Scraper {
    config: SearchScraperConfig,
    mercari_scraper: MercariSearchScraper
}

impl Scraper {
    /// Instantiate a `SearchScraper`.
    pub fn new(config: &SearchScraperConfig) -> Self {
        Self {
            config: config.clone(),
            mercari_scraper: MercariSearchScraper::new()
        }
    }

    /// Attempt to scrape item IDs according to a search criteria.
    /// 
    /// Returns an `Err` for whichever marketplaces had errors while scraping.
    pub async fn scrape_search(&mut self, gallery: &GallerySearchScrapingState) -> HashMap<Marketplace, Result<Vec<ItemId>, String>> {
        tracing::debug!("Starting scrape search for gallery {}", gallery.gallery_id);

        let results = join_all(
            gallery.marketplace_previous_scraped_datetimes
                .clone()
                .iter()
                .map(|(marketplace, previous_scraped_item_datetime)| async {
                    let result = match marketplace {
                        Marketplace::Mercari => self.mercari_scraper
                            .request(&gallery.search_criteria, *previous_scraped_item_datetime)
                            .await
                    };
                    match &result {
                        Ok(ids) => tracing::debug!(
                            "Gallery {}, marketplace {}: scraped {} item IDs", 
                            gallery.gallery_id, *marketplace, ids.len()
                        ),
                        Err(err) => tracing::debug!(
                            "Gallery {}, marketplace {} encountered error: {}", 
                            gallery.gallery_id, *marketplace, err
                        )
                    };
                    (*marketplace, result)
                })
            ).await;

        results
            .into_iter()
            .collect()
    }
}