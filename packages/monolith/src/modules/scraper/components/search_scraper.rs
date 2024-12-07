use std::{collections::HashMap, sync::Arc};

use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use tokio::{sync::Mutex, task::JoinHandle};
use crate::{config::ScraperConfig, galleries::{domain_types::{GalleryId, ItemId, Marketplace, UnixUtcDateTime}, scraping_pipeline::GalleryScrapingState, search_criteria::GallerySearchCriteria}};

use super::state_manager::GalleryStates;

/// The request form sent to the Scrapyd spider for scraping individual items.
#[derive(Serialize, Deserialize, Clone, Debug)]
struct SearchScraperRequestForm {
    pub project: String,
    pub spider: String,
    pub gallery_id: GalleryId,
    pub search_criteria: GallerySearchCriteria,
    pub up_to: UnixUtcDateTime
}

/// This scraper is in charge of using item IDs to scrape detailed data for each item.
pub(super) struct SearchScraper {
    config: ScraperConfig,
    request_client: Client,
    requests_in_progress: HashMap<(GalleryId, Marketplace), JoinHandle<()>>
}

impl SearchScraper {
    /// Instantiate a `SearchScraper`.
    pub fn new(config: &ScraperConfig) -> Self {
        SearchScraper {
            config: config.clone(),
            request_client: Client::new(),
            requests_in_progress: HashMap::new()
        }
    }

    /// Schedules scraping jobs to scrape each marketplace's search within the gallery.
    pub async fn schedule_scrape_search(
        &mut self, 
        gallery: &GalleryScrapingState, 
        gallery_states: Arc<Mutex<GalleryStates>>
    ) {   
        let requests = self.build_requests(gallery);
        for request in requests {
            self.spawn_request(
                gallery.gallery_id.clone(), 
                request.0, 
                request.1, 
                gallery_states.clone()
            );
        }
    }

    /// Build requests to the search scraper for a gallery.
    fn build_requests(&self, gallery: &GalleryScrapingState) -> Vec<(Marketplace, RequestBuilder)> {
        gallery.marketplaces
            .iter()
            .map(|marketplace| {
                let spider_name = match marketplace {
                    Marketplace::Mercari => self.config.mercari_search_spider_name.clone(),
                };
                let req_form = SearchScraperRequestForm {
                    project: self.config.project_name.clone(),
                    spider: spider_name,
                    gallery_id: gallery.gallery_id.clone(),
                    search_criteria: gallery.search_criteria.clone(),
                    up_to: gallery.previous_scraped_item_datetime.clone()
                };
                let req_url = format!("https://{}{}", self.config.scraper_addr, self.config.scraper_scheduling_endpoint);
                let request = self.request_client
                    .post(&req_url)
                    .json(&req_form);
                (marketplace.clone(), request)
            })
            .collect()
    }

    /// Spawns a task to send a HTTP request to the scraper, retrying with exponential backoff until it succeeds.
    /// 
    /// The spawned task handle is added to `self.requests_in_progress`.
    /// 
    /// When successful, it updates its `(GalleryId, Marketplace)` status in `gallery_states`.
    fn spawn_request(
        &mut self, 
        gallery_id: GalleryId,
        marketplace: Marketplace,
        request: RequestBuilder,
        gallery_states: Arc<Mutex<GalleryStates>>
    ) {
        let request_key = (gallery_id.clone(), marketplace.clone());
        let request_handle = tokio::spawn(async move {
            // TODO: implement retry for the request here
            let attempt = request.send().await;
            if gallery_states
                .lock()
                .await
                .update_status(&gallery_id, &marketplace)
                .is_err() {
                    tracing::error!("Attempted to update status for gallery {gallery_id}, marketplace {marketplace} after successful scraping request, but it doesn't exist");
                }
        });
        self.requests_in_progress.insert(request_key, request_handle);
    }
    
}