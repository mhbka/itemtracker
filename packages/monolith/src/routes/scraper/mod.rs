mod mercari;

use axum::Router;
use crate::{config::AxumConfig, modules::AppModuleConnections};

/// Build the router for ingesting data from the scraper. 
/// 
/// TODO: Only allow requests to these routes from whitelisted IPs provided from AxumConfig.
/// 
/// TODO 2: Make the whitelisted IPs settable at runtime
pub(super) fn build_scraper_router(config: &AxumConfig, module_connections: &AppModuleConnections) -> Router {
    let mercari_router = mercari::build_mercari_router(config, module_connections);

    Router::new()
        .nest("/mercari", mercari_router)
}