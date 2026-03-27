use axum::extract::{Path, State};
use http::StatusCode;

use gitdot_api::endpoint::webhook::list_webhooks as api;
use gitdot_core::dto::{ListWebhooksRequest, RepositoryAuthorizationRequest, RepositoryPermission};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_webhooks(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> Result<AppResponse<api::ListWebhooksResponse>, AppError> {
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

    let request = ListWebhooksRequest::new(&owner, &repo)?;
    state
        .webhook_service
        .list_webhooks(request)
        .await
        .map_err(AppError::from)
        .map(|d| AppResponse::new(StatusCode::OK, d.into_api()))
}
