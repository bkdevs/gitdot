use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::get_repository_file as api;
use gitdot_core::dto::{
    GetRepositoryFileRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::AuthenticatedUser,
};

#[axum::debug_handler]
pub async fn get_repository_file(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<api::GetRepositoryFileRequest>,
) -> Result<AppResponse<api::GetRepositoryFileResponse>, AppError> {
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

    let request = GetRepositoryFileRequest::new(&repo, &owner, params.ref_name, params.path)?;
    state
        .repo_service
        .get_repository_file(request)
        .await
        .map_err(AppError::from)
        .map(|file| AppResponse::new(StatusCode::OK, file.into_api()))
}
