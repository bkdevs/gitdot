use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::migration::list_github_installation_repositories as api;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_github_installation_repositories(
    _auth_user: Principal<User>,
    State(state): State<AppState>,
    Path(installation_id): Path<i64>,
) -> Result<AppResponse<api::ListGitHubInstallationRepositoriesResponse>, AppError> {
    state
        .migration_service
        .list_github_installation_repositories(installation_id)
        .await
        .map_err(AppError::from)
        .map(|repos| AppResponse::new(StatusCode::OK, repos.into_api()))
}
