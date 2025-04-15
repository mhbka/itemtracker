use axum::Router;

use crate::app_state::AppState;

/// Build gallery session-related routes.
pub fn build_routes() -> Router<AppState> {
    Router::new()
}