use axum::{routing::post, Json, Router, http::StatusCode};
use crate::{config::AxumConfig, messages::{message_types::scraper::{IngestScrapedItems, IngestScrapedItemsMessage, IngestScrapedSearch, IngestScrapedSearchMessage, ScraperError, ScraperMessage, StartScrapingGallery, StartScrapingGalleryMessage}, ScraperSender}, modules::AppModuleConnections};

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
    router = router.route("/ingest_search", post(
        move |body| ingest_item_ids(body, scraper_sender)
    ));

    let scraper_sender = module_connections.scraper.0.clone();
    router = router.route("/ingest_items", post(
        move |body| ingest_items(body, scraper_sender)
    ));

    router
}

#[tracing::instrument(skip(sender))]
async fn start_scrape(
    Json(data): Json<StartScrapingGallery>,
    mut sender: ScraperSender
) -> Result<StatusCode, (StatusCode, String)> {
    let (msg, response_receiver) = StartScrapingGalleryMessage::new(data);
    sender.send(ScraperMessage::StartScrapingGallery(msg)).await.unwrap();
    match response_receiver.await
    {
        Ok(res) => match res {
            Ok(_) => Ok(StatusCode::OK),
            Err(err) => (Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}"))))
        },
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{err:?}"))),
    }
}

/// Handler for ingesting scraped Mercari item IDs and passing them to the scraper module to be scraped.
/// 
/// TODO: Return a nicer error type?
#[tracing::instrument(skip(sender))]
async fn ingest_item_ids(
    Json(data): Json<IngestScrapedSearch>,
    mut sender: ScraperSender
) -> Result<StatusCode, (StatusCode, String)> {
    let (msg, response_receiver) = IngestScrapedSearchMessage::new(data);

    let send_res = sender.send(ScraperMessage::IngestScrapedSearch(msg)).await;  
    if let Err(err) = send_res {
        let err_str = format!("Critical error: Unable to send a message through ScraperSender ({err:?})");
        tracing::error!("{err_str}");
        panic!("{err_str}");
    }

    match response_receiver.await {
        Ok(response) => {
            match response {
                Ok(_) => Ok(StatusCode::OK),
                Err(err) => {
                    match err {
                        ScraperError::IngestScrapedItemsError { gallery_id, marketplace, error } => {
                            tracing::error!("Error while sending message (gallery: {gallery_id}, {marketplace}): {error})");
                            Err((StatusCode::INTERNAL_SERVER_ERROR, "Scrape could not be requested successfully".into()))
                        },
                        other => {
                            tracing::error!("Unexpected error received: {other}");
                            Err((StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error occurred".into()))
                        },
                    }
                },
            }
        }
        Err(err) => {
            tracing::error!("Error while trying to receive a response for a message sent to the scraper: {err}");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal error occurred".into()))
        }
    }
}

/// Ingests scraped Mercari item data from the route and passes it to the scraper module.
/// 
/// TODO: Return a nicer error type?
#[tracing::instrument(skip(sender))]
async fn ingest_items(
    Json(data): Json<IngestScrapedItems>,
    mut sender: ScraperSender
) -> Result<StatusCode, (StatusCode, String)> {
    let (msg, response_receiver) = IngestScrapedItemsMessage::new(data);

    let send_res = sender.send(ScraperMessage::IngestScrapedItems(msg)).await;  
    if let Err(err) = send_res {
        let err_str = format!("Critical error: Unable to send a message through ScraperSender ({err:?})");
        tracing::error!("{err_str}");
        panic!("{err_str}");
    }

    match response_receiver.await {
        Ok(response) => {
            match response {
                Ok(_) => Ok(StatusCode::OK),
                Err(err) => {
                    match err {
                        ScraperError::IngestScrapedItemsError { gallery_id, marketplace, error } => {
                            tracing::error!("Error while sending message (gallery: {gallery_id}, {marketplace}): {error})");
                            Err((StatusCode::INTERNAL_SERVER_ERROR, "Scrape could not be requested successfully".into()))
                        },
                        other => {
                            tracing::error!("Unexpected error received: {other}");
                            Err((StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error occurred".into()))
                        },
                    }
                },
            }
        }
        Err(err) => {
            tracing::error!("Error while trying to receive a response for a message sent to the scraper: {err:?})");
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal error occurred".into()))
        }
    }
}