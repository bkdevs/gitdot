use std::convert::Infallible;

use axum::{extract::FromRequestParts, http::request::Parts};

pub struct UserAgent(pub Option<String>);

impl<S: Send + Sync> FromRequestParts<S> for UserAgent {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let ua = parts
            .headers
            .get("user-agent")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_string());

        Ok(UserAgent(ua))
    }
}
