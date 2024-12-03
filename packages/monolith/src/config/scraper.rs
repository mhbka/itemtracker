use std::env::{self, VarError};

use serde::{Deserialize, Serialize};

/// Config for the scraper module.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScraperConfig {
    /// Address of the Scrapyd instance.
    pub scraper_addr: String,
    /// Endpoint on the address for scheduling scraping tasks. 
    pub scraper_scheduling_endpoint: String,
    /// The Scrapyd project containing our spiders.
    pub project_name: String,
    /// The name of the spider for hitting Mercari search.
    pub mercari_search_spider_name: String,
    /// The name of the spider for hitting individual Mercari items.
    pub mercari_indiv_spider_name: String
}

impl ScraperConfig {
    /// Load the config from env vars. Returns a `VarError` if any are missing.
    pub(super) fn load() -> Result<Self, VarError> {
        Ok(
            ScraperConfig {
                scraper_addr: env::var("SCRAPER_ADDR")?,
                scraper_scheduling_endpoint: env::var("SCRAPER_SCHEDULING_ENDPOINT")?,
                project_name: env::var("SCRAPER_PROJECT_NAME")?,
                mercari_search_spider_name: env::var("MERCARI_SEARCH_SPIDER_NAME")?,
                mercari_indiv_spider_name: env::var("MERCARI_INDIV_SPIDER_NAME")?
            }
        )
    }
}