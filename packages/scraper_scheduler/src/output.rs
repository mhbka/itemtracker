use reqwest::{self, Client};
use crate::{config::AppConfig, galleries::MercariGallery};

/// Wrapper for outputting to the scraper.
/// TODO: logging
pub struct OutputRequester {
    client: Client,
    config: AppConfig
}

impl OutputRequester {
    pub fn new(config: AppConfig) -> Self {
        OutputRequester {
            client: Client::new(),
            config
        }
    }
    
    /// Send a request for Mercari gallery to the scraper.
    pub async fn schedule_mercari_task(&mut self, gallery: MercariGallery) {
        let response = self.client.post(&self.config.scraper_addr)
            .json(&gallery)
            .send()
            .await;
        match response {
            Ok(_) => println!("successfully scheduled job for gallery {}", gallery.gallery_id),
            Err(err) => eprintln!("error scheduling job for gallery {} ({:?})", gallery.gallery_id, err),
        } 
    }
}