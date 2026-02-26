use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::migration::github::create_github_installation as api;
use gitdot_core::dto::CreateGitHubInstallationRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn create_github_installation(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path(installation_id): Path<i64>,
) -> Result<AppResponse<api::CreateGitHubInstallationResponse>, AppError> {
    let request = CreateGitHubInstallationRequest::new(installation_id, auth_user.id);
    state
        .migration_service
        .create_github_installation(request)
        .await
        .map_err(AppError::from)
        .map(|installation| AppResponse::new(StatusCode::CREATED, installation.into_api()))
}
