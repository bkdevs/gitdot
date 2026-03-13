use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::get_repository_paths as api;
use gitdot_core::dto::{
    GetRepositoryPathsRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn get_repository_paths(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<api::GetRepositoryPathsRequest>,
) -> Result<AppResponse<api::GetRepositoryPathsResponse>, AppError> {
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

    let request = GetRepositoryPathsRequest::new(&repo, &owner, params.ref_name)?;
    state
        .repo_service
        .get_repository_paths(request)
        .await
        .map_err(AppError::from)
        .map(|paths| AppResponse::new(StatusCode::OK, paths.into_api()))
}
