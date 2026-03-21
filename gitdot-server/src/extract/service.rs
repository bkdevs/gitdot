use std::marker::PhantomData;

use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};

use gitdot_core::{dto::JwtClaims, error::AuthorizationError};

use crate::app::{AppError, AppState};

// TODO: this should be mtls or handled at gateway routes? (e.g., /api, /internal)
// use case of certain APIs we only intend for the website to call
pub struct Service<V: Authenticator> {
    _marker: PhantomData<V>,
}

impl<V, S> FromRequestParts<S> for Service<V>
where
    AppState: FromRef<S>,
    V: Authenticator,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        V::authenticate(parts, &app_state)
            .await
            .map_err(AppError::from)?;
        Ok(Service {
            _marker: PhantomData,
        })
    }
}

#[async_trait]
pub trait Authenticator: Send + Sync + 'static {
    async fn authenticate(parts: &Parts, app_state: &AppState) -> Result<(), AuthorizationError>;
}

pub struct Vercel;

#[async_trait]
impl Authenticator for Vercel {
    async fn authenticate(parts: &Parts, app_state: &AppState) -> Result<(), AuthorizationError> {
        let token = parts
            .headers
            .get("x-vercel-oidc-token")
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthorizationError::MissingHeader)?;

        let jwt_header =
            decode_header(token).map_err(|e| AuthorizationError::InvalidToken(e.to_string()))?;
        let kid = jwt_header
            .kid
            .ok_or(AuthorizationError::InvalidToken("missing kid".to_string()))?;

        let jwk = app_state
            .vercel_jwks
            .find(&kid)
            .ok_or(AuthorizationError::InvalidToken(format!(
                "no matching key for kid: {kid}"
            )))?;

        let key = DecodingKey::from_jwk(jwk)
            .map_err(|e| AuthorizationError::InvalidPublicKey(e.to_string()))?;

        let issuer = &app_state.settings.vercel_oidc_url;
        let audience = issuer.replace("oidc.vercel.com", "vercel.com");

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&audience]);
        validation.set_issuer(&[issuer]);

        decode::<JwtClaims>(token, &key, &validation)
            .map_err(|e| AuthorizationError::InvalidToken(e.to_string()))?;

        Ok(())
    }
}
