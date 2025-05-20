use axum::{extract::{Path, State}, routing::{delete, get, patch, post}, Json, Router};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use uuid::Uuid;
use crate::{app_state::AppState, auth::types::AuthUser, domain::{domain_types::ValidCronString, eval_criteria::EvaluationCriteria, gallery::{Gallery, GalleryStats}, search_criteria::SearchCriteria}, models::gallery::{NewGallery, UpdatedGallery}};
use super::error::{RouteError, RouteResult};

#[derive(Deserialize, Debug)]
struct NewGalleryRequest {
    pub name: String,
    pub scraping_periodicity: ValidCronString,
    pub search_criteria: SearchCriteria,
    pub evaluation_criteria: EvaluationCriteria,
    pub mercari_last_scraped_time: Option<NaiveDateTime>,
}

impl NewGalleryRequest {
    fn map_to_model(self, user_id: Uuid) -> NewGallery {
        NewGallery {
            user_id,
            name: self.name,
            scraping_periodicity: self.scraping_periodicity,
            search_criteria: self.search_criteria,
            evaluation_criteria: self.evaluation_criteria,
            mercari_last_scraped_time: self.mercari_last_scraped_time,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Serialize, Debug)]
struct NewGalleryResponse {
    new_gallery_id: Uuid
}

/// Build gallery-related routes.
pub fn build_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(add_new_gallery))
        .route("/:gallery_id", get(get_gallery))
        .route("/:gallery_id", patch(update_gallery))
        .route("/:gallery_id", delete(delete_gallery))
        .route("/stats/:gallery_id", get(get_gallery_stats))
        .route("/stats/all", get(get_all_gallery_stats))
}

async fn add_new_gallery(
    State(mut app_state): State<AppState>,
    user: AuthUser,
    Json(new_gallery): Json<NewGalleryRequest>
) -> RouteResult<Json<NewGalleryResponse>> {
    let mut gallery_store = app_state.stores.gallery_store;
    
    let new_gallery = gallery_store
        .add_new_gallery(new_gallery.map_to_model(user.id))
        .await?;
    let new_gallery_id = new_gallery.id.clone();

    let pipeline_result = app_state.pipeline
        .add_gallery(new_gallery.to_scheduler_state())
        .await;
    if let Err(err) = pipeline_result {
        // if it failed to register in the pipeline, remove from the store too
        gallery_store
            .delete_gallery(new_gallery_id)
            .await?;
        Err(err)?
    }

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
    State(mut app_state): State<AppState>,
    Path(gallery_id): Path<Uuid>,
    user: AuthUser,
    Json(gallery_changes): Json<UpdatedGallery>
) -> RouteResult<()> {
    let mut gallery_store = app_state.stores.gallery_store;

    if gallery_store.gallery_belongs_to_user(gallery_id, user.id).await? {
        let updated_gallery = gallery_store
            .update_gallery(gallery_id, gallery_changes)
            .await?;

        // HACK: if gallery's pipeline is currently running, this needs to wait till it finishes (to acquire the lock).
        // thus we just let it run on another task and return immediately
        tokio::spawn(async move {
            if let Err(err) = app_state.pipeline
                .update_gallery(updated_gallery.to_scheduler_state())
                .await
            {
                tracing::warn!("Error updating gallery {gallery_id} in the scheduler: {err}");
            }
        });

        Ok(())
    }
    else {
        return Err(RouteError::Unauthorized);
    }
}

async fn delete_gallery(
    State(mut app_state): State<AppState>,
    user: AuthUser,
    Path(gallery_id): Path<Uuid>
) -> RouteResult<()> {
    let mut gallery_store = app_state.stores.gallery_store;

    if gallery_store.gallery_belongs_to_user(gallery_id, user.id).await? {
        gallery_store
            .delete_gallery(gallery_id)
            .await?;

        // HACK: if gallery's pipeline is currently running, this needs to wait till it finishes (to acquire the lock).
        // thus we just let it run on another task and return immediately
        tokio::spawn(async move {
            if let Err(err) = app_state.pipeline
                .delete_gallery(gallery_id.into())
                .await
            {
                tracing::warn!("Error removing gallery {gallery_id} from the scheduler: {err}");
            }
        });

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