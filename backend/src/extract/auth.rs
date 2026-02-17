use std::{convert::Infallible, marker::PhantomData};

use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
    http::request::Parts,
};
use base64::Engine;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use gitdot_core::{dto::ValidateTokenRequest, error::AuthorizationError};

use crate::app::{AppError, AppState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthenticatedUser<S: AuthScheme = Any> {
    pub id: Uuid,
    _marker: PhantomData<S>,
}

impl<S: AuthScheme> AuthenticatedUser<S> {
    fn new(id: Uuid) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
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

#[async_trait]
pub trait AuthScheme: Send + Sync + 'static {
    async fn authenticate(
        parts: &Parts,
        app_state: &AppState,
    ) -> Result<AuthenticatedUser<Self>, AuthorizationError>
    where
        Self: Sized;
}

pub struct Any;

#[async_trait]
impl AuthScheme for Any {
    async fn authenticate(
        parts: &Parts,
        app_state: &AppState,
    ) -> Result<AuthenticatedUser<Self>, AuthorizationError> {
        let header = extract_auth_header(parts)?;

        if header.starts_with("Bearer ") {
            let jwt_user = Jwt::authenticate(parts, app_state).await?;
            return Ok(AuthenticatedUser::new(jwt_user.id));
        }
        if header.starts_with("Basic ") {
            let token_user = Token::authenticate(parts, app_state).await?;
            return Ok(AuthenticatedUser::new(token_user.id));
        }

        Err(AuthorizationError::InvalidHeaderFormat)
    }
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

pub struct Jwt;

#[async_trait]
impl AuthScheme for Jwt {
    async fn authenticate(
        parts: &Parts,
        app_state: &AppState,
    ) -> Result<AuthenticatedUser<Self>, AuthorizationError> {
        let header = extract_auth_header(parts)?;
        let jwt = header
            .strip_prefix("Bearer ")
            .ok_or(AuthorizationError::InvalidHeaderFormat)?;
        let mut validation = Validation::new(Algorithm::ES256);
        validation.set_audience(&["authenticated"]);

        let key = DecodingKey::from_ec_pem(app_state.settings.supabase_jwt_public_key.as_bytes())
            .map_err(|e| AuthorizationError::InvalidPublicKey(e.to_string()))?;
        let jwt_data = decode::<UserClaims>(jwt, &key, &validation)
            .map_err(|e| AuthorizationError::InvalidToken(e.to_string()))?;
        let id = Uuid::parse_str(&jwt_data.claims.sub)
            .map_err(|e| AuthorizationError::InvalidToken(e.to_string()))?;

        Ok(AuthenticatedUser::new(id))
    }
}

pub struct Token;

#[async_trait]
impl AuthScheme for Token {
    async fn authenticate(
        parts: &Parts,
        app_state: &AppState,
    ) -> Result<AuthenticatedUser<Self>, AuthorizationError> {
        let header = extract_auth_header(parts)?;
        let token = header
            .strip_prefix("Basic ")
            .ok_or(AuthorizationError::InvalidHeaderFormat)?;

        let decoded = base64::engine::general_purpose::STANDARD
            .decode(token)
            .map_err(|e| AuthorizationError::InvalidToken(e.to_string()))?;
        let token_str = String::from_utf8(decoded)
            .map_err(|e| AuthorizationError::InvalidToken(e.to_string()))?;

        let (_username, token) =
            token_str
                .split_once(':')
                .ok_or(AuthorizationError::InvalidToken(
                    "Invalid token format".to_string(),
                ))?;

        let request = ValidateTokenRequest {
            token: token.to_owned(),
        };
        let response = app_state
            .token_service
            .validate_token(request)
            .await
            .map_err(|_| AuthorizationError::Unauthorized)?;

        Ok(AuthenticatedUser::new(response.user_id))
    }
}

fn extract_auth_header(parts: &Parts) -> Result<&str, AuthorizationError> {
    parts
        .headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AuthorizationError::MissingHeader)
}
