use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::get_repository_commit_stat as api;
use gitdot_core::dto::{GetRepositoryCommitStatRequest, RepositoryAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState, AuthenticatedUser},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn get_repository_commit_stat(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path((owner, repo, sha)): Path<(String, String, String)>,
) -> Result<AppResponse<api::GetRepositoryCommitStatResponse>, AppError> {
    let request = RepositoryAuthorizationRequest::new(auth_user.map(|u| u.id), &owner, &repo)?;
    state
        .auth_service
        .verify_authorized_for_repository(request)
        .await?;

    let request = GetRepositoryCommitStatRequest::new(&repo, &owner, sha)?;
    state
        .repo_service
        .get_repository_commit_stat(request)
        .await
        .map_err(AppError::from)
        .map(|stats| AppResponse::new(StatusCode::OK, stats.into_api()))
}
