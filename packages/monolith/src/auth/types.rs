use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Information about an authenticated user, present in their request's bearer token.
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: Uuid,
    pub exp: usize,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthTokenData {
    pub access_token: String,
    pub refresh_token: String,
    pub user: AuthUser,
}