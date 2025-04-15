use axum::{extract::{Path, State}, routing::{delete, get, patch, post}, Json, Router};
use serde::Serialize;
use uuid::Uuid;
use crate::{app_state::AppState, auth::types::AuthUser, domain::gallery::{Gallery, GalleryStats}, models::gallery::{NewGallery, UpdatedGallery}};
use super::error::{RouteError, RouteResult};

#[derive(Serialize, Debug)]
struct NewGalleryResponse {
    new_gallery_id: Uuid
}


/// Build gallery-related routes.
pub fn build_routes() -> Router<AppState> {
    Router::new()
        .route("/gallery", post(add_new_gallery))
        .route("/gallery/:gallery_id", get(get_gallery))
        .route("/gallery/:gallery_id", patch(update_gallery))
        .route("/gallery/:gallery_id", delete(delete_gallery))
        .route("/gallery_stats/:gallery_id", get(get_gallery_stats))
        .route("/gallery_stats/all", get(get_all_gallery_stats))
}

async fn add_new_gallery(
    State(app_state): State<AppState>,
    user: AuthUser,
    Json(mut new_gallery): Json<NewGallery>
) -> RouteResult<Json<NewGalleryResponse>> {
    let mut gallery_store = app_state.stores.gallery_store;

    new_gallery.user_id = user.id; // in case a different user's ID was given -_-
    let new_gallery_id = gallery_store.add_new_gallery(new_gallery).await?;

    Ok(Json(NewGalleryResponse { new_gallery_id }))
}

async fn get_gallery(
    State(app_state): State<AppState>,
    Path(gallery_id): Path<Uuid>,
    user: AuthUser,
) -> RouteResult<Json<Gallery>> {
    let mut gallery_store = app_state.stores.gallery_store;

    if gallery_store.gallery_belongs_to_user(gallery_id, user.id).await? {
        let gallery = gallery_store.get_gallery(gallery_id).await?;
        return Ok(Json(gallery));
    }
    else {
        return Err(RouteError::Unauthorized);
    }
}

async fn update_gallery( 
    State(app_state): State<AppState>,
    Path(gallery_id): Path<Uuid>,
    user: AuthUser,
    Json(gallery_changes): Json<UpdatedGallery>
) -> RouteResult<()> {
    let mut gallery_store = app_state.stores.gallery_store;

    if gallery_store.gallery_belongs_to_user(gallery_id, user.id).await? {
        gallery_store.update_gallery(gallery_id, gallery_changes).await?;

        // TODO: update in the scheduler too

        Ok(())
    }
    else {
        return Err(RouteError::Unauthorized);
    }
}

async fn delete_gallery(
    State(app_state): State<AppState>,
    user: AuthUser,
    Path(gallery_id): Path<Uuid>
) -> RouteResult<()> {
    let mut gallery_store = app_state.stores.gallery_store;

    if gallery_store.gallery_belongs_to_user(gallery_id, user.id).await? {
        gallery_store.delete_gallery(gallery_id).await?;
        
        // TODO: remove from the scheduler

        return Ok(());
    }
    else {
        return Err(RouteError::Unauthorized);
    }
}

async fn get_gallery_stats(
    State(app_state): State<AppState>,
    user: AuthUser,
    Path(gallery_id): Path<Uuid>
) -> RouteResult<Json<GalleryStats>> {
    let mut gallery_store = app_state.stores.gallery_store;

    if gallery_store.gallery_belongs_to_user(gallery_id, user.id).await? {
        let stats = gallery_store.get_stats(gallery_id).await?;
        return Ok(Json(stats));
    }
    else {
        return Err(RouteError::Unauthorized);
    }
}

async fn get_all_gallery_stats(
    State(app_state): State<AppState>,
    user: AuthUser,
) -> RouteResult<Json<Vec<(Uuid, GalleryStats)>>> {
    let mut gallery_store = app_state.stores.gallery_store;

    let results = gallery_store.get_all_gallery_stats(user.id).await?;

    Ok(Json(results))
}