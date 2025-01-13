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
    let msg = StartScrapingGalleryMessage::new(data);
    sender.send(ScraperMessage::StartScrapingGallery(msg)).await.unwrap();
    Ok(StatusCode::OK)
}

/// Handler for ingesting scraped Mercari item IDs and passing them to the scraper module to be scraped.
/// 
/// TODO: Return a nicer error type?
#[tracing::instrument(skip(sender))]
async fn ingest_item_ids(
    Json(data): Json<IngestScrapedSearch>,
    mut sender: ScraperSender
) -> Result<StatusCode, (StatusCode, String)> {
    let msg = IngestScrapedSearchMessage::new(data);
    let send_res = sender.send(ScraperMessage::IngestScrapedSearch(msg)).await;  
    if let Err(err) = send_res {
        let err_str = format!("Critical error: Unable to send a message through ScraperSender ({err:?})");
        tracing::error!("{err_str}");
        panic!("{err_str}");
    }
    Ok(StatusCode::OK)
}

/// Ingests scraped Mercari item data from the route and passes it to the scraper module.
/// 
/// TODO: Return a nicer error type?
#[tracing::instrument(skip(sender))]
async fn ingest_items(
    Json(data): Json<IngestScrapedItems>,
    mut sender: ScraperSender
) -> Result<StatusCode, (StatusCode, String)> {
    let msg = IngestScrapedItemsMessage::new(data);
    let send_res = sender.send(ScraperMessage::IngestScrapedItems(msg)).await;  
    if let Err(err) = send_res {
        let err_str = format!("Critical error: Unable to send a message through ScraperSender ({err:?})");
        tracing::error!("{err_str}");
        panic!("{err_str}");
    }
    Ok(StatusCode::OK)
}