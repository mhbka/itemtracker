//! This module holds handlers for messages received by the module.
//! 
//! The only reason for putting these here is to make the module file itself neater.
use crate::messages::message_types::scraper::{IngestScrapedItems, IngestScrapedSearch, StartScrapingGallery};
use super::ScraperModule;

pub(super) async fn handle_start_scraping_gallery_msg(msg: StartScrapingGallery, module: &mut ScraperModule) {
    tracing::trace!("Received message to start scraping gallery {}", msg.gallery.gallery_id);
    let schedule_result = module.state_manager
        .start_scraping_gallery(msg.gallery)
        .await;
    if let Err(err) = schedule_result {
        tracing::error!("Error(s) scheduling scraping tasks ({err:#?})");
    };
}

pub(super) async fn handle_ingest_scraped_search_msg(msg: IngestScrapedSearch, module: &mut ScraperModule) {
    tracing::trace!("Received message to ingest scraped search for gallery {} ({})", msg.gallery_id, msg.marketplace);
    let schedule_result = module.state_manager
        .ingest_scraped_search(msg)
        .await;
    if let Err(err) = schedule_result {
        tracing::error!("Error(s) scheduling scraping tasks ({err:#?})");
    };
}

pub(super) async fn handle_ingest_scraped_items_msg(msg: IngestScrapedItems, module: &mut ScraperModule) {
    tracing::trace!("Received message to ingest scraped items for gallery {} ({})", msg.gallery_id, msg.marketplace);
    let schedule_result = module.state_manager
        .ingest_scraped_items(msg)
        .await;
    if let Err(err) = schedule_result {
        tracing::error!("Error(s) scheduling scraping tasks ({err:#?})");
    };
}