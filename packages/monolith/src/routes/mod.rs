mod galleries;
mod gallery_sessions;
mod items;
mod users;
mod error;

use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use crate::app_state::AppState;

pub fn build_router(app_state: AppState) -> Router {
    let galleries_router = galleries::build_routes();
    let gallery_sessions_router = gallery_sessions::build_routes();
    let items_router = items::build_routes();
    let users_router = users::build_routes();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .nest("/g", galleries_router)
        .nest("/s", gallery_sessions_router)
        .nest("/i", items_router)
        .nest("/u", users_router)
        .with_state(app_state)
        .layer(cors)
}   