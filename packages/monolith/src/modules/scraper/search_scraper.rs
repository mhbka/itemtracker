use std::collections::HashMap;
use chrono::{DateTime, Utc};
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use crate::{config::ScraperConfig, galleries::{domain_types::{GalleryId, ItemId, Marketplace}, scraping_pipeline::GalleryScrapingState, search_criteria::GallerySearchCriteria}};

/// The request form sent to the Scrapyd spider for scraping individual items.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct SearchScraperRequestForm {
    pub project: String,
    pub spider: String,
    pub gallery_id: GalleryId,
    pub search_criteria: GallerySearchCriteria,
    pub up_to: DateTime<Utc>
}

/// This scraper is in charge of using item IDs to scrape detailed data for each item.
pub(super) struct SearchScraper {
    config: ScraperConfig,
    request_client: Client
}

impl SearchScraper {
    /// Instantiate a `SearchScraper`.
    pub(super) fn new(config: ScraperConfig) -> Self {
        SearchScraper {
            config,
            request_client: Client::new()
        }
    }

    /// Schedules scraping jobs to scrape each marketplace's search within the gallery.
    /// 
    /// If all marketplace scrapes were scheduled successfully, returns an `Ok`.
    /// 
    /// Else, returns an `Err` containing each unsuccessful marketplace and its associated error message.
    pub(super) async fn schedule_scrape_search(&self, gallery: GalleryScrapingState) -> Result<(), Vec<(Marketplace, String)>>
    {   
        let mut marketplace_errors = Vec::new();
        
        for marketplace in gallery.marketplaces {
            let spider_name = match marketplace {
                Marketplace::Mercari => self.config.mercari_search_spider_name.clone(),
            };
            let request_form = SearchScraperRequestForm {
                project: self.config.project_name.clone(),
                spider: spider_name,
                gallery_id: gallery.gallery_id.clone(),
                search_criteria: gallery.search_criteria.clone(),
                up_to: gallery.previous_scraped_item_datetime
            };
            let req_result = self.request_client
                .post(&self.config.scraper_url)
                .form(&request_form)
                .send()
                .await;
            if let Err(err) = req_result {
                marketplace_errors.push((marketplace, err.to_string()));
            }
        }
        
        if marketplace_errors.len() > 0 { 
            return Err(marketplace_errors); 
        }
        Ok(())
    }
}