use axum::response::{Response, IntoResponse};
use reqwest::StatusCode;
use serde_json::json;
use thiserror::Error;
use crate::stores::error::StoreError;

/// An alias for a direct response of a route handler.
pub type RouteResult<T: IntoResponse> = Result<T, RouteError>;

/// Errors that may return from route handlers.
#[derive(Error, Debug)]
pub enum RouteError {
    #[error("User is not authorized to access this resource")]
    Unauthorized,
    #[error("{0}")]
    Store(#[from] StoreError),
    #[error("Got an error communicating with the pipeline")]
    Pipeline
}

impl IntoResponse for RouteError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::Store(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Encountered an internal error".into()),
            Self::Pipeline => (StatusCode::INTERNAL_SERVER_ERROR, "Encountered an internal error".into()),
        };
        let body = json!({
            "error": message,
        });
        (status, body.to_string()).into_response()
    }
}