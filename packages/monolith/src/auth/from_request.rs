

use super::error::AuthError;
use super::types::{AuthClaims, AuthUser};
use axum::extract::FromRequestParts;
use axum::{
    async_trait,
    http::request::Parts,
};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::{Authorization, HeaderMapExt};
use jsonwebtoken::{decode, DecodingKey, Validation};
use once_cell::sync::Lazy;
use std::env;

/// The JWT secret used for decoding auth-side JWTs.
/// 
/// Must be the same as the one used for encoding them.
static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    env::var("JWT_SECRET").expect("JWT_SECRET must be set")
});

/// Allows us to automatically decode, verify, and extract the user from the request.
/// 
/// Thus, a request handler just needs to have `AuthUser` in the params to authenticate and get the user.
#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let Authorization(bearer) = parts.headers
            .typed_get::<Authorization<Bearer>>()
            .ok_or_else(|| AuthError::Auth)?;

        // HACK: set aud validation to false for now
        let mut validation = Validation::default();
        validation.validate_aud = false;

        // Decode the token
        let token_data = decode::<AuthClaims>(
            bearer.token(),
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &validation,
        )
            .map_err(|e| {
                tracing::info!("Failed to decode: {:?}", e.kind());
                AuthError::InvalidToken
            })?;

        // Extract user data from claims
        let claims = token_data.claims;
        
        Ok(AuthUser {
            id: claims.sub,
            email: claims.email,
            name: claims.name,
            picture: claims.picture,
        })
    }
}