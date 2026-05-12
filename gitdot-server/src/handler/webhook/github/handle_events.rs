use axum::extract::State;
use http::StatusCode;

use gitdot_core::{
    dto::ProcessGithubPushRequest,
    error::{InputError, WebhookError},
};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{GithubEvent, GithubSigned},
};

#[axum::debug_handler]
pub async fn handle_events(
    State(state): State<AppState>,
    GithubSigned {
        event,
        delivery,
        body,
    }: GithubSigned,
) -> Result<AppResponse<()>, AppError> {
    match event {
        GithubEvent::Ping => {
            tracing::info!(%delivery, "github webhook ping acknowledged");
        }
        GithubEvent::Push => {
            let request: ProcessGithubPushRequest = serde_json::from_slice(&body).map_err(|e| {
                WebhookError::Input(InputError::new("github push body", e.to_string()))
            })?;
            state
                .github_webhook_service
                .process_github_push(request)
                .await?;
        }
    }

    Ok(AppResponse::new(StatusCode::OK, ()))
}
