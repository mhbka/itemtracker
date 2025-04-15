use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use serde_json::json;
use thiserror::Error;

/// Possible errors arising from auth.
#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Authentication failed")]
    Auth,
    #[error("Token decoding failed")]
    InvalidToken,
}

/// For immediately returning upon a failed auth
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::Auth => StatusCode::UNAUTHORIZED,
            Self::InvalidToken => StatusCode::UNAUTHORIZED,
        };
        let body = json!({
            "error": self.to_string(),
        });
        (status, body.to_string()).into_response()
    }
}