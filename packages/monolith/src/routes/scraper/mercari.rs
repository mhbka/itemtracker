use axum::{routing::post, Json, Router, http::StatusCode};
use crate::{config::AxumConfig, galleries::pipeline_states::GalleryScrapingState, messages::{message_types::scraper::ScraperMessage, ScraperSender}, modules::AppModuleConnections};

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

    router
}

#[tracing::instrument(skip(sender))]
async fn start_scrape(
    Json(gallery): Json<GalleryScrapingState>,
    mut sender: ScraperSender
) -> Result<StatusCode, (StatusCode, String)> {
    sender.send(ScraperMessage::StartScrapingGallery { gallery }).await.unwrap();
    Ok(StatusCode::OK)
}