use axum::Router;

use crate::app_state::AppState;

/// Build user-related routes.
pub fn build_routes() -> Router<AppState> {
    Router::new()
}