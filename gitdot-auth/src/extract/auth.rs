use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use uuid::Uuid;

use gitdot_core::{
    dto::JwtClaims,
    error::{AuthenticationError, TokenExtractionError},
    util::auth::GITDOT_SERVER_ID,
};

use crate::app::{AppError, AppState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Principal {
    pub id: Uuid,
}

impl<S> FromRequestParts<S> for Principal
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        authenticate(parts, &app_state).map_err(AppError::from)
    }
}

fn authenticate(parts: &Parts, app_state: &AppState) -> Result<Principal, AuthenticationError> {
    let header = parts
        .headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or(TokenExtractionError::MissingHeader)?;

    let jwt = header
        .strip_prefix("Bearer ")
        .ok_or(TokenExtractionError::InvalidHeaderFormat)?;

    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.set_audience(&[GITDOT_SERVER_ID]);

    let key = DecodingKey::from_ed_pem(app_state.settings.gitdot_public_key.as_bytes())
        .map_err(|e| TokenExtractionError::InvalidPublicKey(e.to_string()))?;
    let jwt_data = decode::<JwtClaims>(jwt, &key, &validation)
        .map_err(|e| TokenExtractionError::InvalidToken(e.to_string()))?;
    let id = Uuid::parse_str(&jwt_data.claims.sub)
        .map_err(|e| TokenExtractionError::InvalidToken(e.to_string()))?;

    Ok(Principal { id })
}
