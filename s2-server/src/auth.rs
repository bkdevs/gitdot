use axum::{
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::backend::Backend;

const S2_SERVER_ID: &str = "s2-server";

#[derive(Debug, Clone)]
pub struct Principal {
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: String,
    exp: usize,
    iat: usize,
    aud: Vec<String>,
    iss: String,
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

impl<S> FromRequestParts<S> for Principal
where
    Backend: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let backend = Backend::from_ref(state);

        let header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthError::MissingHeader)?;

        let jwt = header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidHeaderFormat)?;

        let mut validation = Validation::new(Algorithm::EdDSA);
        validation.set_audience(&[S2_SERVER_ID]);

        let key = DecodingKey::from_ed_pem(backend.gitdot_public_key.as_bytes())
            .map_err(|e| AuthError::InvalidPublicKey(e.to_string()))?;

        let jwt_data = decode::<JwtClaims>(jwt, &key, &validation)
            .map_err(|e| AuthError::InvalidToken(e.to_string()))?;

        let id = Uuid::parse_str(&jwt_data.claims.sub)
            .map_err(|e| AuthError::InvalidToken(e.to_string()))?;

        Ok(Principal { id })
    }
}
