use axum::{response::IntoResponse, routing::post, Json, Router, http::{StatusCode, Error}};
use crate::{config::AxumConfig, messages::{message_types::scraper::{ProcessScrapedItems, ProcessScrapedItemsMessage, ScrapeIndivItems, ScrapeIndivItemsMessage, ScraperError, ScraperMessage, StartScrapingJob, StartScrapingJobMessage}, ScraperSender}, modules::AppModuleConnections};

/// Build the routes for ingesting scraped Mercari data.
/// 
/// Consists of 2 routes, 1 for scraped item IDs and 1 for scraped detailed item data.
pub(super) fn build_mercari_router(config: &AxumConfig, module_connections: &AppModuleConnections) -> Router {
    let mut router = Router::new();
    
    // TODO: for testing only, remove later
    let scraper_sender = module_connections.scraper.0.clone();
    router = router.route("/start", post(
        move |body| start_scrape(body, scraper_sender)
    ));

    let scraper_sender = module_connections.scraper.0.clone();
    router = router.route("/search", post(
        move |body| ingest_item_ids(body, scraper_sender)
    ));

    let scraper_sender = module_connections.scraper.0.clone();
    router = router.route("/items", post(
        move |body| ingest_items(body, scraper_sender)
    ));

    router
}

async fn start_scrape(
    Json(data): Json<StartScrapingJob>,
    mut sender: ScraperSender
) -> Result<StatusCode, (StatusCode, String)> {
    let (msg, response_receiver) = StartScrapingJobMessage::new(data);
    sender.send(ScraperMessage::StartScraping(msg)).await.unwrap();
    match response_receiver.await.unwrap() {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, "error".into())),
    }
}

/// Handler for ingesting scraped Mercari item IDs and passing them to the scraper module to be scraped.
/// 
/// TODO: Return a nicer error type?
async fn ingest_item_ids(
    Json(data): Json<ScrapeIndivItems>,
    mut sender: ScraperSender
) -> Result<StatusCode, (StatusCode, String)> {
    let (msg, response_receiver) = ScrapeIndivItemsMessage::new(data);

    let send_res = sender.send(ScraperMessage::ScrapeIndividualItems(msg)).await;  
    if let Err(err) = send_res {
        // TODO: this is pretty critical error; log and panic here (but in the distant future, a graceful restart would be nice)
        panic!("Error while sending message to ScraperSender: {err:?}");
    }

    match response_receiver.await {
        Ok(response) => {
            match response {
                Ok(_) => Ok(StatusCode::OK),
                Err(err) => {
                    match err {
                        ScraperError::UnsuccessfulSearchScrapeRequest { gallery_id, error } => {
                            // TODO: log and return the error
                            Err((StatusCode::INTERNAL_SERVER_ERROR, "Scrape could not be requested successfully".into()))
                        },
                        ScraperError::UnsuccessfulIndivScrapeRequest { gallery_id, marketplace, error_str } => {
                            // TODO: this should not happen
                            Err((StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error occurred".into()))
                        },
                    }
                },
            }
        }
        Err(err) => {
            // TODO: In this case the sender is dropped... log it and return a 500?
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal error occurred".into()))
        }
    }
}

/// Ingests scraped Mercari item data from the route and passes it to the scraper module.
/// 
/// TODO: Return a nicer error type?
async fn ingest_items(
    Json(data): Json<ProcessScrapedItems>,
    mut sender: ScraperSender
) -> Result<StatusCode, (StatusCode, String)> {
    let (msg, response_receiver) = ProcessScrapedItemsMessage::new(data);

    let send_res = sender.send(ScraperMessage::ProcessScrapedItems(msg)).await;  
    if let Err(err) = send_res {
        // TODO: this is pretty critical error; log and panic here (but in the distant future, a graceful restart would be nice)
        panic!("Error while sending message to ScraperSender: {err:?}");
    }

    match response_receiver.await {
        Ok(response) => {
            match response {
                Ok(_) => Ok(StatusCode::OK),
                Err(err) => {
                    match err {
                        ScraperError::UnsuccessfulSearchScrapeRequest { gallery_id, error } => {
                            // TODO: this should not happen
                            Err((StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error occurred".into()))
                        },
                        ScraperError::UnsuccessfulIndivScrapeRequest { gallery_id, marketplace, error_str } => {
                            // TODO: log and return the error
                            Err((StatusCode::INTERNAL_SERVER_ERROR, "Scrape could not be requested successfully".into()))
                        },
                    }
                },
            }
        }
        Err(err) => {
            // TODO: In this case the sender is dropped... log it and return a 500?
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal error occurred".into()))
        }
    }
}