use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_core::dto::{GetRepositoryFileCommitsRequest, RepositoryAuthorizationRequest};

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{GetRepositoryCommitsServerResponse, GetRepositoryFileCommitsQuery};

#[axum::debug_handler]
pub async fn get_repository_file_commits(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<GetRepositoryFileCommitsQuery>,
) -> Result<AppResponse<GetRepositoryCommitsServerResponse>, AppError> {
    let request = RepositoryAuthorizationRequest::new(auth_user.map(|u| u.id), &owner, &repo)?;
    state
        .auth_service
        .verify_authorized_for_repository(request)
        .await?;

    let request = GetRepositoryFileCommitsRequest::new(
        &repo,
        &owner,
        params.ref_name,
        params.path,
        params.page,
        params.per_page,
    )?;
    state
        .repo_service
        .get_repository_file_commits(request)
        .await
        .map_err(AppError::from)
        .map(|commits| AppResponse::new(StatusCode::OK, commits.into()))
}
