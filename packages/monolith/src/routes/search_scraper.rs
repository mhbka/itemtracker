use axum::{routing::post, Json, Router};
use reqwest::StatusCode;
use crate::{config::AxumConfig, domain::pipeline_states::GallerySearchScrapingState, messages::{message_types::search_scraper::SearchScraperMessage, SearchScraperSender}, scraping_pipeline::AppModuleConnections};

/// Build the router for ingesting data from the scraper. 
/// 
/// TODO: Only allow requests to these routes from whitelisted IPs provided from AxumConfig.
/// 
/// TODO 2: Make the whitelisted IPs settable at runtime
pub(super) fn build(config: &AxumConfig, module_connections: &AppModuleConnections) -> Router {
    let mut router = Router::new();

    let scraper_sender = module_connections.search_scraper.0.clone();
    router = router.route("/start", post(
        move |body| start_scrape(body, scraper_sender)
    ));

    router
}

async fn start_scrape(
    Json(gallery): Json<GallerySearchScrapingState>,
    mut sender: SearchScraperSender
) -> Result<StatusCode, (StatusCode, String)> {
    tokio::spawn(async move {
        sender.send(SearchScraperMessage::ScrapeSearchNew { gallery }).await.unwrap();
    });
    Ok(StatusCode::OK)
}