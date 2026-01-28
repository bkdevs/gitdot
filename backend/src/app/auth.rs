use std::convert::Infallible;

use axum::{
    extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
    http::request::Parts,
};
use base64::Engine;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use gitdot_core::error::AuthorizationError;

use super::{AppError, AppState};

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

    if let Some(token) = auth_header.strip_prefix("Bearer ") {
        authenticate_with_jwt(token, app_state)
    } else if let Some(credentials) = auth_header.strip_prefix("Basic ") {
        authenticate_with_credential(credentials, app_state)
    } else {
        Err(AuthorizationError::InvalidHeaderFormat)
    }
}

fn authenticate_with_jwt(
    token: &str,
    app_state: &AppState,
) -> Result<AuthenticatedUser, AuthorizationError> {
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

fn authenticate_with_credential(
    credentials: &str,
    _app_state: &AppState,
) -> Result<AuthenticatedUser, AuthorizationError> {
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(credentials)
        .map_err(|_| AuthorizationError::Unauthorized)?;

    let credential_str =
        String::from_utf8(decoded).map_err(|_| AuthorizationError::Unauthorized)?;

    let (username, _password) = credential_str
        .split_once(':')
        .ok_or(AuthorizationError::Unauthorized)?;

    // TODO: Validate credentials and return user_id
    todo!("Authenticate user '{}' with password/token", username)
}
