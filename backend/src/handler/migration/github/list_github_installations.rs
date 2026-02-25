use axum::{extract::State, http::StatusCode};

use gitdot_api::endpoint::migration::list_github_installations as api;
use gitdot_core::dto::ListGitHubInstallationsRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_github_installations(
    auth_user: Principal<User>,
    State(state): State<AppState>,
) -> Result<AppResponse<api::ListGitHubInstallationsResponse>, AppError> {
    let request = ListGitHubInstallationsRequest::new(auth_user.id);
    state
        .migration_service
        .list_github_installations(request)
        .await
        .map_err(AppError::from)
        .map(|installations| AppResponse::new(StatusCode::OK, installations.into_api()))
}
