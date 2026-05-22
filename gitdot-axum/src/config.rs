//! Typed configuration values consumed by the shared extractors and middleware.
//!
//! Each config is a small `Clone` struct that a host service builds at startup
//! and stores in its `AppState`. By implementing `FromRef` for these types, the
//! service lets `gitdot-axum`'s extractors and middleware pull the config they
//! need straight out of request state.

use std::sync::Arc;

use jsonwebtoken::jwk::JwkSet;

/// Configuration for verifying gitdot-issued JWTs.
///
/// Consumed by the [`Principal`](crate::extract::Principal) extractor, which
/// validates the request's bearer token against [`public_key`](Self::public_key).
#[derive(Clone)]
pub struct AuthConfig {
    /// Ed25519 public key, PEM-encoded, used to verify gitdot JWT signatures.
    pub public_key: String,
}

/// Configuration for verifying Vercel OIDC tokens on incoming requests.
///
/// Consumed by the [`verify_vercel_oidc`](crate::middleware::verify_vercel_oidc)
/// middleware, which authenticates server-to-server requests originating from
/// Vercel deployments.
#[derive(Clone)]
pub struct VercelOidcConfig {
    /// JWKS fetched from Vercel's OIDC endpoint, used to verify token signatures.
    pub jwks: Arc<JwkSet>,
    /// Expected `iss` claim — the Vercel OIDC issuer URL for this project.
    pub issuer: String,
}
