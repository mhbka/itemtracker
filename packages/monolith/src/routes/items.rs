use axum::Router;

use crate::app_state::AppState;

/// Build item-related routes.
pub fn build_routes() -> Router<AppState> {
    Router::new()
}