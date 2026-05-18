use axum::{
    extract::{Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::migration::github::list_github_installations as api;
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
    Query(query): Query<api::ListGitHubInstallationsRequest>,
) -> Result<AppResponse<api::ListGitHubInstallationsResponse>, AppError> {
    let request =
        ListGitHubInstallationsRequest::new(auth_user.id, query.cursor.as_deref(), query.limit)?;
    state
        .migration_service
        .list_github_installations(request)
        .await
        .map_err(AppError::from)
        .map(|page| AppResponse::new(StatusCode::OK, page.into_api()))
}
