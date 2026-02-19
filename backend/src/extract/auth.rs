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

use gitdot_core::{dto::ValidateTokenRequest, error::AuthorizationError, model::token::TokenType};

use crate::app::{AppError, AppState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Principal<S: Authenticator> {
    pub id: Uuid,
    _marker: PhantomData<S>,
}

impl<S: Authenticator> Principal<S> {
    fn new(id: Uuid) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}

impl<A, S> FromRequestParts<S> for Principal<A>
where
    AppState: FromRef<S>,
    A: Authenticator,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        A::authenticate(parts, &app_state)
            .await
            .map_err(AppError::from)
    }
}

impl<A, S> OptionalFromRequestParts<S> for Principal<A>
where
    AppState: FromRef<S>,
    A: Authenticator,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        Ok(A::authenticate(parts, &app_state).await.ok())
    }
}

#[async_trait]
pub trait Authenticator: Send + Sync + 'static {
    async fn authenticate(
        parts: &Parts,
        app_state: &AppState,
    ) -> Result<Principal<Self>, AuthorizationError>
    where
        Self: Sized;
}

pub struct User;

#[async_trait]
impl Authenticator for User {
    async fn authenticate(
        parts: &Parts,
        app_state: &AppState,
    ) -> Result<Principal<Self>, AuthorizationError> {
        let header = extract_auth_header(parts)?;

        if header.starts_with("Bearer ") {
            let jwt_user = UserJwt::authenticate(parts, app_state).await?;
            return Ok(Principal::new(jwt_user.id));
        }
        if header.starts_with("Basic ") {
            let token_user = UserToken::authenticate(parts, app_state).await?;
            return Ok(Principal::new(token_user.id));
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

pub struct UserJwt;

#[async_trait]
impl Authenticator for UserJwt {
    async fn authenticate(
        parts: &Parts,
        app_state: &AppState,
    ) -> Result<Principal<Self>, AuthorizationError> {
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

        Ok(Principal::new(id))
    }
}

pub struct UserToken;

#[async_trait]
impl Authenticator for UserToken {
    async fn authenticate(
        parts: &Parts,
        app_state: &AppState,
    ) -> Result<Principal<Self>, AuthorizationError> {
        let header = extract_auth_header(parts)?;
        let token = extract_token(header)?;

        let request = ValidateTokenRequest {
            token: token.to_owned(),
            token_type: TokenType::Personal,
        };
        let response = app_state
            .auth_service
            .validate_token(request)
            .await
            .map_err(|_| AuthorizationError::Unauthorized)?;

        Ok(Principal::new(response.principal_id))
    }
}

pub struct Runner;

#[async_trait]
impl Authenticator for Runner {
    async fn authenticate(
        parts: &Parts,
        app_state: &AppState,
    ) -> Result<Principal<Self>, AuthorizationError> {
        let header = extract_auth_header(parts)?;
        let token = extract_token(header)?;

        let request = ValidateTokenRequest {
            token: token.to_owned(),
            token_type: TokenType::Runner,
        };
        let response = app_state
            .auth_service
            .validate_token(request)
            .await
            .map_err(|_| AuthorizationError::Unauthorized)?;

        Ok(Principal::new(response.principal_id))
    }
}

fn extract_token(header: &str) -> Result<String, AuthorizationError> {
    let token = header
        .strip_prefix("Basic ")
        .ok_or(AuthorizationError::InvalidHeaderFormat)?;

    let decoded = base64::engine::general_purpose::STANDARD
        .decode(token)
        .map_err(|e| AuthorizationError::InvalidToken(e.to_string()))?;
    let token_str =
        String::from_utf8(decoded).map_err(|e| AuthorizationError::InvalidToken(e.to_string()))?;

    let (_, token) = token_str
        .split_once(':')
        .ok_or(AuthorizationError::InvalidToken(
            "Invalid token format".to_string(),
        ))?;

    Ok(token.to_string())
}

fn extract_auth_header(parts: &Parts) -> Result<&str, AuthorizationError> {
    parts
        .headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(AuthorizationError::MissingHeader)
}
