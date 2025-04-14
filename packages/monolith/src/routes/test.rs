use axum::{routing::post, Json, Router};
use reqwest::StatusCode;
use crate::{config::AxumConfig, domain::pipeline_states::GallerySearchScrapingState, messages::{message_types::search_scraper::SearchScraperMessage, SearchScraperSender}, models::gallery::NewGallery, scraping_pipeline::AppModuleConnections, stores::{galleries::GalleryStore, AppStores}};

pub(super) fn build(
    config: &AxumConfig, 
    module_connections: &AppModuleConnections,
    app_store: &AppStores
) -> Router {
    let mut router = Router::new();

    let scraper_sender = module_connections.search_scraper.0.clone();
    router = router.route("/start", post(
        move |body| start_scrape(body, scraper_sender)
    ));

    let gallery_store = app_store.gallery_store.clone();
    router = router.route("/add_gallery", post(
        move |body| add_gallery(body, gallery_store)
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

async fn add_gallery(
    Json(gallery): Json<NewGallery>,
    mut gallery_store: GalleryStore
) -> Result<StatusCode, (StatusCode, String)> {
    gallery_store
        .add_new_gallery(gallery)
        .await
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;

    Ok(StatusCode::OK)
}