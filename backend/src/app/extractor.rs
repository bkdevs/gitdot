use api::EndpointRequest;
use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Query, Request},
};
use serde_json::Value;

use super::AppError;

pub struct Ext<T>(pub T);

impl<S, T> FromRequest<S> for Ext<T>
where
    S: Send + Sync,
    T: EndpointRequest,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let value = if matches!(
            *req.method(),
            http::Method::GET | http::Method::HEAD | http::Method::DELETE
        ) {
            let (mut parts, _body) = req.into_parts();
            Query::<Value>::from_request_parts(&mut parts, state)
                .await
                .ok()
                .and_then(|Query(v)| match v {
                    Value::Object(map) => Some(Value::Object(map)),
                    _ => None,
                })
                .unwrap_or(Value::Object(serde_json::Map::new()))
        } else {
            Json::<Value>::from_request(req, state)
                .await
                .ok()
                .unwrap_or(Json(Value::Object(serde_json::Map::new())))
                .0
        };

        let request = serde_json::from_value::<T>(value)
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))?;
        Ok(Ext(request))
    }
}
