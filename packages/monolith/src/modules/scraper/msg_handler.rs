//! This module holds handlers for messages received by the module.
//! 
//! The only reason for putting these here is to make the module file itself neater.
use crate::messages::message_types::scraper::{IngestScrapedItemsMessage, IngestScrapedSearchMessage, StartScrapingGalleryMessage};
use super::ScraperModule;

pub(super) async fn handle_start_scraping_gallery_msg(msg: StartScrapingGalleryMessage, module: &mut ScraperModule) {
    let new_gallery = msg.get_msg().gallery;
    let schedule_result = module.state_manager
        .start_scraping_gallery(new_gallery.clone())
        .await;
    if let Err(err) = schedule_result {
        tracing::error!("Error(s) scheduling scraping tasks ({err:#?})");
    };
}

pub(super) async fn handle_ingest_scraped_search_msg(msg: IngestScrapedSearchMessage, module: &mut ScraperModule) {
    let inner_msg = msg.get_msg();
    let schedule_result = module.state_manager
        .ingest_scraped_search(inner_msg.clone())
        .await;
    if let Err(err) = schedule_result {
        tracing::error!("Error(s) scheduling scraping tasks ({err:#?})");
    };
}

pub(super) async fn handle_ingest_scraped_items_msg(msg: IngestScrapedItemsMessage, module: &mut ScraperModule) {
    let inner_msg = msg.get_msg();
    let schedule_result = module.state_manager
        .ingest_scraped_items(inner_msg.clone())
        .await;
    if let Err(err) = schedule_result {
        tracing::error!("Error(s) scheduling scraping tasks ({err:#?})");
    };
}