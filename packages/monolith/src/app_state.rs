use crate::{pipeline::Pipeline, stores::AppStores};

/// The application state.
#[derive(Clone)]
pub struct AppState {
    /// For accessing storage of app data.
    pub stores: AppStores,
    /// For talking to the pipeline.
    pub pipeline: Pipeline
}