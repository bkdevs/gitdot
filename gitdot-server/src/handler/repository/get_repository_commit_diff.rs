use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::repository::get_repository_commit_diff as api;
use gitdot_core::dto::{
    GetCommitDiffRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn get_repository_commit_diff(
    auth_user: Option<Principal<User>>,
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
        .authorization_service
        .verify_authorized_for_repository(request)
        .await?;

    let request = GetCommitDiffRequest::new(&owner, &repo, sha)?;
    state
        .commit_service
        .get_commit_diff(request)
        .await
        .map_err(AppError::from)
        .map(|diff| AppResponse::new(StatusCode::OK, diff.into_api()))
}
