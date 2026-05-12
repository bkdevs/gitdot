use axum::extract::State;
use http::StatusCode;

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{GithubEvent, GithubSigned},
};

#[axum::debug_handler]
pub async fn handle_events(
    State(_state): State<AppState>,
    GithubSigned {
        event,
        delivery,
        body,
    }: GithubSigned,
) -> Result<AppResponse<()>, AppError> {
    tracing::info!(
        event = ?event,
        %delivery,
        body_len = body.len(),
        "received github webhook event",
    );

    match event {
        GithubEvent::Ping => {
            tracing::info!(%delivery, "github webhook ping acknowledged");
        }
        GithubEvent::Push => {
            // TODO: deserialize push payload and dispatch to service
        }
    }

    Ok(AppResponse::new(StatusCode::OK, ()))
}
