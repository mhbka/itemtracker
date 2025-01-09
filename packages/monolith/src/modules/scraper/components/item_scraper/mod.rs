use std::{collections::HashMap, sync::Arc};
use request_form::ItemScraperRequestForm;
use reqwest::{Client, RequestBuilder};
use tokio::{sync::Mutex, task::JoinHandle};
use crate::{config::ScraperConfig, galleries::domain_types::{GalleryId, Marketplace}, messages::message_types::scraper::IngestScrapedSearch};
use super::state_manager::GalleryStates;

mod request_form;

/// This scraper is in charge of scraping detailed data for each item ID.
pub(super) struct ItemScraper { 
    config: ScraperConfig,
    request_client: Client,
    requests_in_progress: HashMap<(GalleryId, Marketplace), JoinHandle<()>>
}

impl ItemScraper {
    /// Instantiate a `IndividualScraper`.
    pub fn new(config: &ScraperConfig) -> Self {
        Self {
            config: config.clone(),
            request_client: Client::new(),
            requests_in_progress: HashMap::new()
        }
    }

    /// Schedule a scraping job for the given item IDs under the given marketplace for detailed item data,
    /// tagging them under the given gallery.
    pub async fn schedule_scrape_items(
        &mut self, 
        data: IngestScrapedSearch,
        gallery_states: Arc<Mutex<GalleryStates>> 
    )
    {   tracing::trace!("Scheduling scraping of items for gallery {} ({})", data.gallery_id, data.marketplace);
        let (gallery_id, marketplace) = (data.gallery_id.clone(), data.marketplace.clone());
        let request = self.build_request(data);
        self.spawn_request(
            gallery_id,
            marketplace,
            request, 
            gallery_states
        );
    }   

    /// Build a HTTP request to the item scraper for a marketplace under a gallery.
    fn build_request(&self, data: IngestScrapedSearch) -> RequestBuilder {
        tracing::trace!("Building request");
        let spider_name = match data.marketplace {
            Marketplace::Mercari => self.config.mercari_indiv_spider_name.clone()
        };
        let req_form = ItemScraperRequestForm {
            project: self.config.project_name.clone(),
            spider: spider_name,
            gallery_id: data.gallery_id,    
            item_ids: data.scraped_item_ids
        };
        let req_url = format!("http://{}{}", self.config.scraper_addr, self.config.scraper_scheduling_endpoint);
        self.request_client
            .post(&req_url)
            .form(&req_form)
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
            tracing::trace!("Attempting request for gallery {gallery_id} ({marketplace})");
            // TODO: implement retry for the request here
            match request.send().await {
                Ok(res) => tracing::trace!("Successfully requested search scrape for gallery {gallery_id} ({marketplace}); response: {}", res.text().await.unwrap()),
                Err(err) => tracing::error!("Failed to request search scrape for gallery {gallery_id} ({marketplace}): {err:#?}")
            }
            if gallery_states
                .lock()
                .await
                .update_status(&gallery_id, &marketplace)
                .is_err() {
                    tracing::error!("Attempted to update status for gallery {gallery_id}, marketplace {marketplace} after successful scraping request, but it doesn't exist");
                }
        });
        tracing::trace!("Adding search scrape task handle for {request_key:?}");
        self.requests_in_progress.insert(request_key, request_handle);
    }
}