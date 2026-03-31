use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{app::error::AppError, consts::REFRESH_TOKEN_COOKIE_NAME};

pub struct RefreshToken(pub String);

impl<S: Send + Sync> FromRequestParts<S> for RefreshToken {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let cookie_header = parts
            .headers
            .get("cookie")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let token = cookie_header
            .split(';')
            .map(|s| s.trim())
            .find_map(|s| s.strip_prefix(&format!("{REFRESH_TOKEN_COOKIE_NAME}=")))
            .ok_or(AppError::Authentication(
                gitdot_core::error::AuthenticationError::SessionNotFound,
            ))?;

        Ok(RefreshToken(token.to_string()))
    }
}
