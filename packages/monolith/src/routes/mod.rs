mod test;

use axum::Router;
use crate::{config::AxumConfig, scraping_pipeline::AppModuleConnections, stores::AppStores};

pub fn build_router(config: &AxumConfig, module_connections: &AppModuleConnections, app_store: &AppStores) -> Router {
    let search_scraper_router = test::build(config, module_connections, app_store);

    Router::new()
        .nest("/scraper", search_scraper_router)
}