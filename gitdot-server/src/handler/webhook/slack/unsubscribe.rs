use axum::extract::{Path, State};
use http::StatusCode;
use uuid::Uuid;

use gitdot_core::dto::{
    RepositoryAuthorizationRequest, RepositoryPermission, UnsubscribeSlackWebhookRequest,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::UnsubscribeSlackWebhookRequest as WireRequest,
    extract::SlackBotSigned,
};

#[axum::debug_handler]
pub async fn unsubscribe_slack_webhook(
    State(state): State<AppState>,
    Path((owner, repo, webhook_id)): Path<(String, String, Uuid)>,
    SlackBotSigned(body): SlackBotSigned<WireRequest>,
) -> Result<AppResponse<()>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(
        Some(body.gitdot_user_id),
        &owner,
        &repo,
        RepositoryPermission::Admin,
    )?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = UnsubscribeSlackWebhookRequest::new(
        webhook_id,
        body.gitdot_user_id,
        &owner,
        &repo,
        body.slack_user_id,
        body.slack_team_id,
        body.slack_channel_id,
    )?;
    state
        .slack_webhook_service
        .unsubscribe_slack_webhook(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::NO_CONTENT, ()))
}
