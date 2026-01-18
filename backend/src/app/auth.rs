use std::convert::Infallible;

use axum::{
    extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use gitdot_core::errors::AuthorizationError;

use crate::app::{AppError, AppState};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserClaims {
    pub sub: String,
    pub email: Option<String>,
    pub exp: usize,
    pub iat: usize,
    pub aud: String,
    pub role: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedUser {
    pub id: Uuid,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        authenticate_user(parts, &app_state).map_err(AppError::from)
    }
}

impl<S> OptionalFromRequestParts<S> for AuthenticatedUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        Ok(authenticate_user(parts, &app_state).ok())
    }
}

fn authenticate_user(
    parts: &Parts,
    app_state: &AppState,
) -> Result<AuthenticatedUser, AuthorizationError> {
    let auth_header = parts
        .headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AuthorizationError::MissingHeader)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AuthorizationError::InvalidHeaderFormat)?;

    let mut validation = Validation::new(Algorithm::ES256);
    validation.set_audience(&["authenticated"]);
    let jwt_public_key = app_state.settings.supabase_jwt_public_key.as_bytes();
    let key = DecodingKey::from_ec_pem(jwt_public_key)
        .map_err(|e| AuthorizationError::InvalidPublicKey(e.to_string()))?;

    let token_data = decode::<UserClaims>(token, &key, &validation)
        .map_err(|e| AuthorizationError::InvalidToken(e.to_string()))?;

    let id = Uuid::parse_str(&token_data.claims.sub)
        .map_err(|e| AuthorizationError::InvalidToken(e.to_string()))?;

    Ok(AuthenticatedUser { id })
}
