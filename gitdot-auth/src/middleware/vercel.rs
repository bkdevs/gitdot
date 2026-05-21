use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};

use gitdot_core::{
    dto::JwtClaims,
    error::{AuthenticationError, TokenExtractionError},
};

use crate::app::{AppError, AppState};

pub async fn verify_vercel_oidc(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = request
        .headers()
        .get("x-vercel-oidc-token")
        .and_then(|v| v.to_str().ok())
        .ok_or(AuthenticationError::Extraction(
            TokenExtractionError::MissingHeader,
        ))?;

    let jwt_header = decode_header(token)
        .map_err(|e| TokenExtractionError::InvalidToken(e.to_string()))
        .map_err(AuthenticationError::Extraction)?;
    let kid = jwt_header
        .kid
        .ok_or(TokenExtractionError::InvalidToken(
            "missing kid".to_string(),
        ))
        .map_err(AuthenticationError::Extraction)?;

    let jwk = state
        .vercel_jwks
        .find(&kid)
        .ok_or(TokenExtractionError::InvalidToken(format!(
            "no matching key for kid: {kid}"
        )))
        .map_err(AuthenticationError::Extraction)?;

    let key = DecodingKey::from_jwk(jwk)
        .map_err(|e| TokenExtractionError::InvalidPublicKey(e.to_string()))
        .map_err(AuthenticationError::Extraction)?;

    let issuer = &state.settings.vercel_oidc_url;
    let audience = issuer.replace("oidc.vercel.com", "vercel.com");

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[&audience]);
    validation.set_issuer(&[issuer]);

    decode::<JwtClaims>(token, &key, &validation)
        .map_err(|e| TokenExtractionError::InvalidToken(e.to_string()))
        .map_err(AuthenticationError::Extraction)?;

    Ok(next.run(request).await)
}
