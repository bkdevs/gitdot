use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_core::dto::{GetRepositoryCommitsRequest, RepositoryAuthorizationRequest};

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{GetRepositoryCommitsQuery, GetRepositoryCommitsServerResponse};

#[axum::debug_handler]
pub async fn get_repository_commits(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<GetRepositoryCommitsQuery>,
) -> Result<AppResponse<GetRepositoryCommitsServerResponse>, AppError> {
    let request =
        RepositoryAuthorizationRequest::with_owner_repo(auth_user.map(|u| u.id), &owner, &repo)?;
    state
        .auth_service
        .verify_authorized_for_repository(request)
        .await?;

    let request = GetRepositoryCommitsRequest::new(
        &repo,
        &owner,
        params.ref_name,
        params.page,
        params.per_page,
    )?;
    state
        .repo_service
        .get_repository_commits(request)
        .await
        .map_err(AppError::from)
        .map(|commits| AppResponse::new(StatusCode::OK, commits.into()))
}
