mod galleries;
mod gallery_sessions;
mod items;
mod users;
mod error;

use axum::Router;
use axum::routing::get;
use tower_http::{cors::{Any, CorsLayer}, trace::TraceLayer};
use crate::app_state::AppState;

pub fn build_router(app_state: AppState) -> Router {
    let galleries_router = galleries::build_routes();
    let gallery_sessions_router = gallery_sessions::build_routes();
    let items_router = items::build_routes();
    let users_router = users::build_routes();

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let trace_layer = TraceLayer::new_for_http();

    Router::new()
        .route("/health", get(|| async {}))
        .nest("/g", galleries_router)
        .nest("/s", gallery_sessions_router)
        .nest("/i", items_router)
        .nest("/u", users_router)
        .with_state(app_state)
        .layer(cors_layer)
        .layer(trace_layer)
}   