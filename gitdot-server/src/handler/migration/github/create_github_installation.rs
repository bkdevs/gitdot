use axum::{
    Json,
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
    Json(body): Json<api::CreateGitHubInstallationRequest>,
) -> Result<AppResponse<api::CreateGitHubInstallationResponse>, AppError> {
    let request =
        CreateGitHubInstallationRequest::new(installation_id, auth_user.id, body.state, body.code);
    state
        .migration_service
        .create_github_installation(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::CREATED, r.into_api()))
}
