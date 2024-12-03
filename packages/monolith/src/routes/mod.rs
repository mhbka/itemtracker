mod scraper;

use axum::Router;
use crate::{config::AxumConfig, modules::AppModuleConnections};

pub fn build_router(config: &AxumConfig, module_connections: &AppModuleConnections) -> Router {
    let scraper_router = scraper::build_scraper_router(config, module_connections);

    Router::new()
        .nest("/scraper", scraper_router)
}