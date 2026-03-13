use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::get_repository_commits as api;
use gitdot_core::dto::{GetCommitsRequest, RepositoryAuthorizationRequest, RepositoryPermission};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

// TODO: this does not support ref in request as of now, service ignores it.
#[axum::debug_handler]
pub async fn get_repository_commits(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<api::GetRepositoryCommitsRequest>,
) -> Result<AppResponse<api::GetRepositoryCommitsResponse>, AppError> {
    let request = RepositoryAuthorizationRequest::new(
        auth_user.map(|u| u.id),
        &owner,
        &repo,
        RepositoryPermission::Read,
    )?;
    state
        .authorization_service
        .verify_authorized_for_repository(request)
        .await?;

    let request =
        GetCommitsRequest::new(&owner, &repo, params.ref_name, params.page, params.per_page)?;
    state
        .commit_service
        .get_commits(request)
        .await
        .map_err(AppError::from)
        .map(|commits| AppResponse::new(StatusCode::OK, commits.into_api()))
}
