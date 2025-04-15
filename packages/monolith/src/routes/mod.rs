mod test;
mod galleries;
mod gallery_sessions;
mod items;
mod users;
mod error;

use axum::Router;
use crate::{app_state::AppState, config::AxumConfig, scraping_pipeline::PipelineConnections, stores::AppStores};

pub fn build_router(app_state: AppState) -> Router {
    let galleries_router = galleries::build_routes();
    let gallery_sessions_router = gallery_sessions::build_routes();
    let items_router = items::build_routes();
    let users_router = users::build_routes();

    Router::new()
        .nest("/g", galleries_router)
        .nest("/s", gallery_sessions_router)
        .nest("/i", items_router)
        .nest("/u", users_router)
        .with_state(app_state)
}   