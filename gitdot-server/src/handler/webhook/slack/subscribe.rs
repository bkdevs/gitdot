use axum::extract::{Path, State};
use http::StatusCode;

use gitdot_core::dto::{
    RepositoryAuthorizationRequest, RepositoryPermission, SubscribeSlackWebhookRequest,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::SubscribeSlackWebhookRequest as WireRequest,
    extract::SlackBotSigned,
};

#[axum::debug_handler]
pub async fn subscribe_slack_webhook(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
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

    let request = SubscribeSlackWebhookRequest::new(
        body.gitdot_user_id,
        &owner,
        &repo,
        body.slack_user_id,
        body.slack_team_id,
        body.slack_channel_id,
    )?;
    state
        .webhook_service
        .subscribe_slack_webhook(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::CREATED, ()))
}
