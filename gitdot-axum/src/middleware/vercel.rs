use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};

use gitdot_core::dto::JwtClaims;

use crate::config::VercelOidcConfig;

pub async fn verify_vercel_oidc(
    State(config): State<VercelOidcConfig>,
    request: Request,
    next: Next,
) -> Response {
    if verify(&request, &config).is_err() {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    next.run(request).await
}

fn verify(request: &Request, config: &VercelOidcConfig) -> Result<(), ()> {
    let token = request
        .headers()
        .get("x-vercel-oidc-token")
        .and_then(|v| v.to_str().ok())
        .ok_or(())?;

    let header = decode_header(token).map_err(|_| ())?;
    let kid = header.kid.ok_or(())?;
    let jwk = config.jwks.find(&kid).ok_or(())?;
    let key = DecodingKey::from_jwk(jwk).map_err(|_| ())?;

    let audience = config.issuer.replace("oidc.vercel.com", "vercel.com");
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[&audience]);
    validation.set_issuer(&[&config.issuer]);

    decode::<JwtClaims>(token, &key, &validation).map_err(|_| ())?;
    Ok(())
}
