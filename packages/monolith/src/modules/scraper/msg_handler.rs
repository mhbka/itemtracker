//! This module holds handlers for messages received by the module.
//! 
//! The only reason for putting these here is to make the module file itself neater.
use crate::messages::message_types::scraper::{IngestScrapedItems, IngestScrapedSearch, StartScrapingGallery};
use super::ScraperModule;

pub(super) async fn handle_start_scraping_gallery_msg(msg: StartScrapingGallery, module: &mut ScraperModule) {
    let schedule_result = module.state_manager
        .start_scraping_gallery(msg.gallery)
        .await;
    if let Err(err) = schedule_result {
        tracing::error!("Error(s) scheduling scraping tasks ({err:#?})");
    };
}

pub(super) async fn handle_ingest_scraped_search_msg(msg: IngestScrapedSearch, module: &mut ScraperModule) {
    let schedule_result = module.state_manager
        .ingest_scraped_search(msg)
        .await;
    if let Err(err) = schedule_result {
        tracing::error!("Error(s) scheduling scraping tasks ({err:#?})");
    };
}

pub(super) async fn handle_ingest_scraped_items_msg(msg: IngestScrapedItems, module: &mut ScraperModule) {
    let schedule_result = module.state_manager
        .ingest_scraped_items(msg)
        .await;
    if let Err(err) = schedule_result {
        tracing::error!("Error(s) scheduling scraping tasks ({err:#?})");
    };
}