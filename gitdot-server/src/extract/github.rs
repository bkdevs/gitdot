use std::str::FromStr;

use axum::{
    body::Body,
    extract::{FromRef, FromRequest, Request},
    http::request::Parts,
};
use uuid::Uuid;

use gitdot_core::{
    dto::VerifyGithubSignatureRequest,
    error::{AuthenticationError, InputError, WebhookError},
};

use crate::app::{AppError, AppState};

const GITHUB_EVENT_HEADER: &str = "X-GitHub-Event";
const GITHUB_DELIVERY_HEADER: &str = "X-GitHub-Delivery";
const GITHUB_SIGNATURE_HEADER: &str = "X-Hub-Signature-256";

const MAX_BODY_BYTES: usize = 5 * 1024 * 1024;

pub struct GithubSigned {
    pub event: GithubEvent,
    pub delivery: Uuid,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GithubEvent {
    Ping,
    Push,
}

impl FromStr for GithubEvent {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ping" => Ok(Self::Ping),
            "push" => Ok(Self::Push),
            other => Err(format!("unsupported github event: {other}")),
        }
    }
}

impl<S> FromRequest<S> for GithubSigned
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);
        let (parts, body) = req.into_parts();

        let event_str = header_value(&parts, GITHUB_EVENT_HEADER)?;
        let delivery_str = header_value(&parts, GITHUB_DELIVERY_HEADER)?;
        let signature = header_value(&parts, GITHUB_SIGNATURE_HEADER)?;
        let body_bytes = read_body(body).await?;
        let request = VerifyGithubSignatureRequest::new(body_bytes.clone(), signature);
        app_state
            .authentication_service
            .verify_github_signature(request)?;

        let event = GithubEvent::from_str(&event_str)
            .map_err(|reason| WebhookError::Input(InputError::new(GITHUB_EVENT_HEADER, reason)))?;
        let delivery = Uuid::parse_str(&delivery_str).map_err(|e| {
            WebhookError::Input(InputError::new(GITHUB_DELIVERY_HEADER, e.to_string()))
        })?;

        Ok(GithubSigned {
            event,
            delivery,
            body: body_bytes,
        })
    }
}

fn header_value(parts: &Parts, name: &'static str) -> Result<String, AppError> {
    parts
        .headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .ok_or_else(|| AuthenticationError::Unauthorized.into())
}

async fn read_body(body: Body) -> Result<Vec<u8>, AppError> {
    let bytes = axum::body::to_bytes(body, MAX_BODY_BYTES)
        .await
        .map_err(|_| AuthenticationError::Unauthorized)?;
    Ok(bytes.to_vec())
}
