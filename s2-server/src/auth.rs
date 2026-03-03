use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use uuid::Uuid;

use crate::backend::Backend;

const S2_SERVER_ID: &str = "s2-server";
const GITDOT_SERVER_ID: &str = "gitdot-server";

#[derive(Debug, Clone)]
pub struct Principal<A: Authenticator> {
    pub id: Uuid,
    _marker: PhantomData<A>,
}

impl<A: Authenticator> Principal<A> {
    fn new(id: Uuid) -> Self {
        Self {
            id,
            _marker: PhantomData,
        }
    }
}
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("missing authorization header")]
    MissingHeader,
    #[error("invalid authorization header format")]
    InvalidHeaderFormat,
    #[error("invalid token: {0}")]
    InvalidToken(String),
    #[error("invalid public key: {0}")]
    InvalidPublicKey(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, self.to_string()).into_response()
    }
}

#[async_trait]
pub trait Authenticator: Send + Sync + 'static {
    async fn authenticate(parts: &Parts, backend: &Backend) -> Result<Principal<Self>, AuthError>
    where
        Self: Sized;
}

impl<A, S> FromRequestParts<S> for Principal<A>
where
    Backend: FromRef<S>,
    A: Authenticator,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let backend = Backend::from_ref(state);
        A::authenticate(parts, &backend).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: String,
    exp: usize,
    iat: usize,
    aud: Vec<String>,
    iss: String,
}

fn extract_jwt(parts: &Parts) -> Result<&str, AuthError> {
    let header = parts
        .headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AuthError::MissingHeader)?;
    header
        .strip_prefix("Bearer ")
        .ok_or(AuthError::InvalidHeaderFormat)
}

pub struct Internal;

#[async_trait]
impl Authenticator for Internal {
    async fn authenticate(parts: &Parts, backend: &Backend) -> Result<Principal<Self>, AuthError> {
        let jwt = extract_jwt(parts)?;
        let claims = decode_jwt(jwt, &backend.gitdot_public_key)?;
        if claims.sub != GITDOT_SERVER_ID {
            return Err(AuthError::InvalidToken(
                "expected internal server identity".to_string(),
            ));
        }
        Ok(Principal::new(Uuid::nil()))
    }
}

pub struct TaskJwt;

#[async_trait]
impl Authenticator for TaskJwt {
    async fn authenticate(parts: &Parts, backend: &Backend) -> Result<Principal<Self>, AuthError> {
        let jwt = extract_jwt(parts)?;
        let claims = decode_jwt(jwt, &backend.gitdot_public_key)?;
        let id =
            Uuid::parse_str(&claims.sub).map_err(|e| AuthError::InvalidToken(e.to_string()))?;
        Ok(Principal::new(id))
    }
}

fn decode_jwt(jwt: &str, public_key: &str) -> Result<JwtClaims, AuthError> {
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.set_audience(&[S2_SERVER_ID]);
    let key = DecodingKey::from_ed_pem(public_key.as_bytes())
        .map_err(|e| AuthError::InvalidPublicKey(e.to_string()))?;
    decode::<JwtClaims>(jwt, &key, &validation)
        .map(|d| d.claims)
        .map_err(|e| AuthError::InvalidToken(e.to_string()))
}
