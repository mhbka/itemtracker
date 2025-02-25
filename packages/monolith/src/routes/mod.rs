mod search_scraper;

use axum::Router;
use crate::{config::AxumConfig, scraping_pipeline::AppModuleConnections};

pub fn build_router(config: &AxumConfig, module_connections: &AppModuleConnections) -> Router {
    let search_scraper_router = search_scraper::build(config, module_connections);

    Router::new()
        .nest("/scraper", search_scraper_router)
}