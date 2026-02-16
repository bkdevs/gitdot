use std::convert::Infallible;

use axum::{extract::FromRequestParts, http::header, http::request::Parts};

pub struct ContentType(pub String);

impl<S: Send + Sync> FromRequestParts<S> for ContentType {
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let value = parts
            .headers
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        Ok(ContentType(value))
    }
}
