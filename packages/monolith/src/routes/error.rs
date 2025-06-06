use std::error::Error;
use axum::response::{Response, IntoResponse};
use reqwest::StatusCode;
use serde_json::json;
use thiserror::Error;
use crate::{pipeline::scheduler::error::SchedulerError, stores::error::StoreError};

/// An alias for a direct response of a route handler.
pub type RouteResult<T> = Result<T, RouteError>;

/// Errors that may return from route handlers.
#[derive(Error, Debug)]
pub enum RouteError {
    #[error("User is not authorized to access this resource")]
    Unauthorized,
    #[error("Got an error from the store ({0})")]
    Store(#[from] StoreError),
    #[error("Got an error communicating with the pipeline")]
    Pipeline(#[from] SchedulerError)
}

impl IntoResponse for RouteError {
    fn into_response(self) -> Response {
        tracing::info!("Responding with error: {} (source: {:?})", self, self.source());

        let (status, message) = match self {
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::Store(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Encountered an internal error".into()),
            Self::Pipeline(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Encountered an internal error".into()),
        };
        let body = json!({
            "error": message,
        });
        (status, body.to_string()).into_response()
    }
}