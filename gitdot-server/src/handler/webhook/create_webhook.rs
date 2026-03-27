use axum::extract::{Json, Path, State};
use http::StatusCode;

use gitdot_api::endpoint::webhook::create_webhook as api;
use gitdot_core::dto::{
    CreateWebhookRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn create_webhook(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Json(request): Json<api::CreateWebhookRequest>,
) -> Result<AppResponse<api::CreateWebhookResponse>, AppError> {
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

    let request =
        CreateWebhookRequest::new(&owner, &repo, &request.url, request.secret, request.events)?;
    state
        .webhook_service
        .create_webhook(request)
        .await
        .map_err(AppError::from)
        .map(|d| AppResponse::new(StatusCode::CREATED, d.into_api()))
}
