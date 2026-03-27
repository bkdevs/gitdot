use axum::extract::{Json, Path, State};
use http::StatusCode;
use uuid::Uuid;

use gitdot_api::endpoint::webhook::update_webhook as api;
use gitdot_core::dto::{
    RepositoryAuthorizationRequest, RepositoryPermission, UpdateWebhookRequest,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn update_webhook(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, webhook_id)): Path<(String, String, Uuid)>,
    Json(request): Json<api::UpdateWebhookRequest>,
) -> Result<AppResponse<api::UpdateWebhookResponse>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(
        Some(auth_user.id),
        &owner,
        &repo,
        RepositoryPermission::Admin,
    )?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = UpdateWebhookRequest::new(
        &owner,
        &repo,
        webhook_id,
        request.url.as_deref(),
        request.secret,
        request.events,
    )?;
    state
        .webhook_service
        .update_webhook(request)
        .await
        .map_err(AppError::from)
        .map(|d| AppResponse::new(StatusCode::OK, d.into_api()))
}
