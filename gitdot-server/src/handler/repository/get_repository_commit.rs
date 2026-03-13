use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::get_repository_commit as api;
use gitdot_core::dto::{GetCommitRequest, RepositoryAuthorizationRequest, RepositoryPermission};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn get_repository_commit(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo, sha)): Path<(String, String, String)>,
) -> Result<AppResponse<api::GetRepositoryCommitResponse>, AppError> {
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

    let request = GetCommitRequest::new(&owner, &repo, sha)?;
    state
        .commit_service
        .get_commit(request)
        .await
        .map_err(AppError::from)
        .map(|commit| AppResponse::new(StatusCode::OK, commit.into_api()))
}
