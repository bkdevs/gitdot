use std::convert::Infallible;

use axum::{extract::FromRequestParts, http::request::Parts};

pub struct ClientCookies {
    pub sha: Option<String>,
    pub timestamp: Option<String>,
}

impl<S: Send + Sync> FromRequestParts<S> for ClientCookies {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let sha = parts
            .headers
            .get("X-Gitdot-Client-Sha")
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned);
        let timestamp = parts
            .headers
            .get("X-Gitdot-Client-Timestamp")
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned);
        Ok(ClientCookies { sha, timestamp })
    }
}
