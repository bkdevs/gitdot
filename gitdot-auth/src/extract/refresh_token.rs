use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::extract::CookieJar;

use gitdot_core::error::AuthenticationError;

use crate::{app::error::AppError, consts::REFRESH_TOKEN_COOKIE_NAME};

pub struct RefreshToken(pub String);

impl<S: Send + Sync> FromRequestParts<S> for RefreshToken {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .expect("CookieJar extraction is infallible");

        let token = jar
            .get(REFRESH_TOKEN_COOKIE_NAME)
            .map(|c| c.value().to_string())
            .ok_or(AppError::Authentication(
                AuthenticationError::SessionNotFound,
            ))?;

        Ok(RefreshToken(token))
    }
}
