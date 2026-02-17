use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::AuthenticatedUser,
};
use gitdot_api::endpoint::get_repository_commit_diff as api;
use gitdot_core::dto::{
    GetRepositoryCommitDiffRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

#[axum::debug_handler]
pub async fn get_repository_commit_diff(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path((owner, repo, sha)): Path<(String, String, String)>,
) -> Result<AppResponse<api::GetRepositoryCommitDiffResponse>, AppError> {
    let request = RepositoryAuthorizationRequest::new(
        auth_user.map(|u| u.id),
        &owner,
        &repo,
        RepositoryPermission::Read,
    )?;
    state
        .auth_service
        .verify_authorized_for_repository(request)
        .await?;

    let request = GetRepositoryCommitDiffRequest::new(&repo, &owner, sha)?;
    state
        .repo_service
        .get_repository_commit_diff(request)
        .await
        .map_err(AppError::from)
        .map(|diffs| AppResponse::new(StatusCode::OK, diffs.into_api()))
}
