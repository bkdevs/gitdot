use std::convert::Infallible;
use std::marker::PhantomData;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedUser<S: AuthScheme = Either> {
    pub id: Uuid,
    _marker: PhantomData<S>,
}

impl<Scheme, S> FromRequestParts<S> for AuthenticatedUser<Scheme>
where
    Scheme: AuthScheme,
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        Scheme::authenticate(parts, &app_state)
            .await
            .map_err(AppError::from)
    }
}

impl<Scheme, S> OptionalFromRequestParts<S> for AuthenticatedUser<Scheme>
where
    Scheme: AuthScheme,
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        Ok(Scheme::authenticate(parts, &app_state).await.ok())
    }
}

pub struct Jwt;
pub struct Token;
pub struct Either;

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::Jwt {}
    impl Sealed for super::Token {}
    impl Sealed for super::Either {}
}

pub trait AuthScheme: sealed::Sealed + Send + Sync + 'static {
    fn authenticate(
        parts: &Parts,
        app_state: &AppState,
    ) -> impl std::future::Future<Output = Result<AuthenticatedUser<Self>, AuthorizationError>> + Send
    where
        Self: Sized;
}

impl AuthScheme for Jwt {
    async fn authenticate(
        parts: &Parts,
        app_state: &AppState,
    ) -> Result<AuthenticatedUser<Self>, AuthorizationError> {
        let header = extract_auth_header(parts)?;
        let id = authenticate_with_jwt(header, app_state)?;
        Ok(AuthenticatedUser {
            id,
            _marker: PhantomData,
        })
    }
}

impl AuthScheme for Token {
    async fn authenticate(
        parts: &Parts,
        app_state: &AppState,
    ) -> Result<AuthenticatedUser<Self>, AuthorizationError> {
        let header = extract_auth_header(parts)?;
        let id = authenticate_with_token(header, app_state).await?;
        Ok(AuthenticatedUser {
            id,
            _marker: PhantomData,
        })
    }
}

impl AuthScheme for Either {
    async fn authenticate(
        parts: &Parts,
        app_state: &AppState,
    ) -> Result<AuthenticatedUser<Self>, AuthorizationError> {
        let header = extract_auth_header(parts)?;
        let id = if header.starts_with("Bearer ") {
            authenticate_with_jwt(header, app_state)?
        } else if header.starts_with("Basic ") {
            authenticate_with_token(header, app_state).await?
        } else {
            return Err(AuthorizationError::InvalidHeaderFormat);
        };
        Ok(AuthenticatedUser {
            id,
            _marker: PhantomData,
        })
    }
}

fn extract_auth_header(parts: &Parts) -> Result<&str, AuthorizationError> {
    parts
        .headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AuthorizationError::MissingHeader)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserClaims {
    pub sub: String,
    pub email: Option<String>,
    pub exp: usize,
    pub iat: usize,
    pub aud: String,
    pub role: Option<String>,
}

fn authenticate_with_jwt(header: &str, app_state: &AppState) -> Result<Uuid, AuthorizationError> {
    let jwt = header
        .strip_prefix("Bearer ")
        .ok_or(AuthorizationError::InvalidHeaderFormat)?;

    let mut validation = Validation::new(Algorithm::ES256);
    validation.set_audience(&["authenticated"]);
    let jwt_public_key = app_state.settings.supabase_jwt_public_key.as_bytes();
    let key = DecodingKey::from_ec_pem(jwt_public_key)
        .map_err(|e| AuthorizationError::InvalidPublicKey(e.to_string()))?;

    let jwt_data = decode::<UserClaims>(jwt, &key, &validation)
        .map_err(|e| AuthorizationError::InvalidToken(e.to_string()))?;

    Uuid::parse_str(&jwt_data.claims.sub)
        .map_err(|e| AuthorizationError::InvalidToken(e.to_string()))
}

async fn authenticate_with_token(
    header: &str,
    app_state: &AppState,
) -> Result<Uuid, AuthorizationError> {
    let token = header
        .strip_prefix("Basic ")
        .ok_or(AuthorizationError::InvalidHeaderFormat)?;

    let decoded = base64::engine::general_purpose::STANDARD
        .decode(token)
        .map_err(|_| AuthorizationError::Unauthorized)?;

    let token = String::from_utf8(decoded).map_err(|_| AuthorizationError::Unauthorized)?;
    let (_username, token) = token
        .split_once(':')
        .ok_or(AuthorizationError::Unauthorized)?;

    app_state
        .token_service
        .validate_token(token)
        .await
        .map_err(|_| AuthorizationError::Unauthorized)
}
