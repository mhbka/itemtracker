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
    match schedule_result {
        Ok(_) => {
            let response_res = msg.respond(Ok(()));
            if let Err(err) = response_res {
                tracing::error!("
                    Was unable to send a response for a message...
                    Message: {new_gallery:#?},
                    Response: {err:#?}
                ");
            }
        },
        Err(err) => {
            tracing::error!("Error(s) scheduling scraping tasks ({err:#?})");
            let response_res = msg.respond(Err(err));
            if let Err(err) = response_res {
                tracing::error!("
                    Was unable to send a response for a message...
                    Message: {new_gallery:#?},
                    Response: {err:#?}
                ");
            }
        }
    };
}

pub(super) async fn handle_ingest_scraped_search_msg(msg: IngestScrapedSearchMessage, module: &mut ScraperModule) {
    let inner_msg = msg.get_msg();
    let schedule_result = module.state_manager
        .ingest_scraped_search(inner_msg.clone())
        .await;
    match schedule_result {
        Ok(_) => {
            let response_res = msg.respond(Ok(()));
            if let Err(err) = response_res {
                tracing::error!("
                    Was unable to send a response for a message...
                    Message: {inner_msg:#?},
                    Response: {err:#?}
                ");
            }
        },
        Err(err) => {
            tracing::error!("Error(s) scheduling scraping tasks ({err:#?})");
            let response_res = msg.respond(Err(err));
            if let Err(err) = response_res {
                tracing::error!("
                    Was unable to send a response for a message...
                    Message: {inner_msg:#?},
                    Response: {err:#?}
                ");
            }
        }
    };
}

pub(super) async fn handle_ingest_scraped_items_msg(msg: IngestScrapedItemsMessage, module: &mut ScraperModule) {
    let inner_msg = msg.get_msg();
    let schedule_result = module.state_manager
        .ingest_scraped_items(inner_msg.clone())
        .await;
    match schedule_result {
        Ok(_) => {
            let response_res = msg.respond(Ok(()));
            if let Err(err) = response_res {
                tracing::error!("
                    Was unable to send a response for a message...
                    Message: {inner_msg:#?},
                    Response: {err:#?}
                ");
            }
        },
        Err(err) => {
            tracing::error!("Error(s) scheduling scraping tasks ({err:#?})");
            let response_res = msg.respond(Err(err));
            if let Err(err) = response_res {
                tracing::error!("
                    Was unable to send a response for a message...
                    Message: {inner_msg:#?},
                    Response: {err:#?}
                ");
            }
        }
    };
}