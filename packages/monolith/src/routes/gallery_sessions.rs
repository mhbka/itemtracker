use axum::{extract::{Path, State}, routing::{delete, get, patch, post}, Json, Router};
use crate::{app_state::AppState, auth::types::AuthUser, domain::gallery_session::{GallerySession, GallerySessionStats}};

use super::error::{RouteError, RouteResult};

/// Build gallery session-related routes.
pub fn build_routes() -> Router<AppState> {
    Router::new()
        .route("/:session_id", get(get_gallery_session))
        .route("/stats/:session_id", get(get_gallery_session_stats))
}

async fn get_gallery_session(
    State(app_state): State<AppState>,
    Path(session_id): Path<i32>,
    user: AuthUser,
) -> RouteResult<Json<GallerySession>> {
    let mut gallery_sessions_store = app_state.stores.gallery_sessions_store;

    if gallery_sessions_store.session_belongs_to_user(session_id, user.id).await? {
        let session = gallery_sessions_store.get_session(session_id).await?;
        return Ok(Json(session));
    }
    else {
        return Err(RouteError::Unauthorized);
    }
}

async fn get_gallery_session_stats(
    State(app_state): State<AppState>,
    Path(session_id): Path<i32>,
    user: AuthUser,
) -> RouteResult<Json<GallerySessionStats>> {
    let mut gallery_sessions_store = app_state.stores.gallery_sessions_store;

    if gallery_sessions_store.session_belongs_to_user(session_id, user.id).await? {
        let stats = gallery_sessions_store.get_session_stats(session_id).await?;
        return Ok(Json(stats));
    }
    else {
        return Err(RouteError::Unauthorized);
    }
}