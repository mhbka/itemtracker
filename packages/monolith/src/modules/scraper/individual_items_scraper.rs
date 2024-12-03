use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::{config::ScraperConfig, galleries::{domain_types::{GalleryId, ItemId}, domain_types::Marketplace}};

/// The request form sent to the Scrapyd spider for scraping individual items.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct IndividualScraperRequestForm {
    pub project: String,
    pub spider: String,
    pub gallery_id: GalleryId,
    pub item_ids: Vec<ItemId>
}

/// This scraper is in charge of using item IDs to scrape detailed data for each item.
pub(super) struct IndividualItemsScraper {
    config: ScraperConfig,
    request_client: Client
}

impl IndividualItemsScraper {
    /// Instantiate a `IndividualScraper`.
    pub(super) fn new(config: ScraperConfig) -> Self {
        IndividualItemsScraper {
            config,
            request_client: Client::new()
        }
    }

    /// Schedule a scraping job for the given item IDs under the given marketplace for detailed item data,
    /// tagging them under the given gallery.
    /// 
    /// `Ok` is returned once successfully scheduled, but it doesn't mean the job has been successfully executed.
    /// 
    /// Scraped data will eventually be returned through the input message bus.
    pub(super) async fn schedule_scrape_items(
        &self,
        gallery_id: GalleryId,
        marketplace: Marketplace,
        scraped_item_ids: Vec<ItemId>
    ) -> Result<(), reqwest::Error>
    {   
        let spider_name = match marketplace {
            Marketplace::Mercari => self.config.mercari_indiv_spider_name.clone()
        };
        let req_form = IndividualScraperRequestForm {
            project: self.config.project_name.clone(),
            spider: spider_name,
            gallery_id,
            item_ids: scraped_item_ids
        };
        let req_url = format!("{}{}", self.config.scraper_addr, self.config.scraper_scheduling_endpoint);
        self.request_client
            .post(&req_url)
            .form(&req_form)
            .send()
            .await?; 
        // TODO: Maybe designate custom internal errors for the module, then map to it
        // and also implement `reqwest_retry`
        
        Ok(())
    }
}