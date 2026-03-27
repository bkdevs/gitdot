use axum::extract::{Path, State};
use http::StatusCode;
use uuid::Uuid;

use gitdot_api::endpoint::webhook::get_webhook as api;
use gitdot_core::dto::{GetWebhookRequest, RepositoryAuthorizationRequest, RepositoryPermission};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn get_webhook(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, webhook_id)): Path<(String, String, Uuid)>,
) -> Result<AppResponse<api::GetWebhookResponse>, AppError> {
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

    let request = GetWebhookRequest::new(&owner, &repo, webhook_id)?;
    state
        .webhook_service
        .get_webhook(request)
        .await
        .map_err(AppError::from)
        .map(|d| AppResponse::new(StatusCode::OK, d.into_api()))
}
