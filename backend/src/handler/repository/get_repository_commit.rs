use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    app::{AppError, AppResponse, AppState, AuthenticatedUser},
    dto::IntoApi,
};
use api::endpoint::get_repository_commit as api;
use gitdot_core::dto::{GetRepositoryCommitRequest, RepositoryAuthorizationRequest};

#[axum::debug_handler]
pub async fn get_repository_commit(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path((owner, repo, sha)): Path<(String, String, String)>,
) -> Result<AppResponse<api::GetRepositoryCommitResponse>, AppError> {
    let request = RepositoryAuthorizationRequest::new(auth_user.map(|u| u.id), &owner, &repo)?;
    state
        .auth_service
        .verify_authorized_for_repository(request)
        .await?;

    let request = GetRepositoryCommitRequest::new(&repo, &owner, sha)?;
    state
        .repo_service
        .get_repository_commit(request)
        .await
        .map_err(AppError::from)
        .map(|commits| AppResponse::new(StatusCode::OK, commits.into_api()))
}
