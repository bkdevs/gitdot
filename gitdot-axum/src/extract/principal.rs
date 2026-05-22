use std::convert::Infallible;

use axum::{
    extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
    http::{HeaderMap, StatusCode, request::Parts},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use uuid::Uuid;

use gitdot_core::{dto::JwtClaims, util::auth::GITDOT_SERVER_ID};

use crate::config::AuthConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Principal {
    pub id: Uuid,
}

impl<S> FromRequestParts<S> for Principal
where
    AuthConfig: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = AuthConfig::from_ref(state);
        authenticate(&parts.headers, &config).map_err(|_| StatusCode::UNAUTHORIZED)
    }
}

impl<S> OptionalFromRequestParts<S> for Principal
where
    AuthConfig: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let config = AuthConfig::from_ref(state);
        Ok(authenticate(&parts.headers, &config).ok())
    }
}

fn authenticate(headers: &HeaderMap, config: &AuthConfig) -> Result<Principal, ()> {
    let header = headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(())?;

    let jwt = header.strip_prefix("Bearer ").ok_or(())?;

    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.set_audience(&[GITDOT_SERVER_ID]);

    let key = DecodingKey::from_ed_pem(config.public_key.as_bytes()).map_err(|_| ())?;
    let jwt_data = decode::<JwtClaims>(jwt, &key, &validation).map_err(|_| ())?;
    let id = Uuid::parse_str(&jwt_data.claims.sub).map_err(|_| ())?;

    Ok(Principal { id })
}
