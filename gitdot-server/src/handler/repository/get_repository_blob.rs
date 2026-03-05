use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::get_repository_blob as api;
use gitdot_core::dto::{
    GetRepositoryBlobRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn get_repository_blob(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<api::GetRepositoryBlobRequest>,
) -> Result<AppResponse<api::GetRepositoryBlobResponse>, AppError> {
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

    let request = GetRepositoryBlobRequest::new(&repo, &owner, params.ref_name, params.path)?;
    state
        .repo_service
        .get_repository_blob(request)
        .await
        .map_err(AppError::from)
        .map(|blob| AppResponse::new(StatusCode::OK, blob.into_api()))
}
