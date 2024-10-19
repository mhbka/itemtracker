use std::sync::Arc;
use tokio::sync::Mutex;
use axum::{extract::{Json, State}, http::StatusCode, routing::{get, post}, Router};
use crate::{galleries::{Galleries, MercariGallery}, state::AppState};

/// The main function for building the service router.
pub fn build_routes(app_state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .route("/add_mercari_gallery", post(add_mercari_gallery_handler))
        .route("/edit_mercari_gallery", post(edit_mercari_gallery_handler))
        .route("/ping", get(ping_handler))
        .with_state(app_state)
}

/// Handles adding Mercari galleries.
/// 
/// An added gallery immediately starts its task and returns an `OK`.
async fn add_mercari_gallery_handler(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(payload_gallery): Json<MercariGallery>
) -> (StatusCode, String) {
    let app_state = &mut app_state
        .lock()
        .await;
    match app_state.add_gallery(Galleries::Mercari(payload_gallery)).await {
        Ok(()) => (StatusCode::OK, "".into()),
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Error scheduling job: {e}"))
        }
    }
}

/// Handles editing Mercari galleries.
/// 
/// An edited gallery **starts its next schedule on the current schedule**. 
/// 
/// For example, if, based on the current schedule,the next task is in 12 hours, 
/// and the edited gallery's new schedule is every 6 hours, this schedule will only start in 12 hours.
/// 
/// Returns `ACCEPTED` if the input is valid, though the task may still fail later on.
async fn edit_mercari_gallery_handler(
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(payload_gallery): Json<MercariGallery>
) -> (StatusCode, String) {
    let app_state = &mut app_state
        .lock()
        .await;
    match app_state.edit_gallery(Galleries::Mercari(payload_gallery)).await {
        Ok(()) => (StatusCode::ACCEPTED, "".into()), // edited gallery doesn't run immediately, so OK is not appropriate
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error scheduling job: {e}"))
    }
}

/// Ping!
async fn ping_handler() -> &'static str {
    "ping!"
}
