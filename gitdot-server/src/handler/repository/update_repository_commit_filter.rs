use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_api::endpoint::repository::update_repository_commit_filter as api;
use gitdot_core::dto::{
    RepositoryAuthorizationRequest, RepositoryPermission, UpdateRepositoryCommitFilterRequest,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn update_repository_commit_filter(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, filter_id)): Path<(String, String, Uuid)>,
    Json(request): Json<api::UpdateRepositoryCommitFilterRequest>,
) -> Result<AppResponse<api::UpdateRepositoryCommitFilterResponse>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(
        Some(auth_user.id),
        &owner,
        &repo,
        RepositoryPermission::Write,
    )?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let core_request = UpdateRepositoryCommitFilterRequest::new(
        filter_id,
        &request.name,
        request.authors,
        request.tags,
        request.paths,
    )?;
    state
        .repo_service
        .update_repository_commit_filter(core_request)
        .await
        .map_err(AppError::from)
        .map(|filter| AppResponse::new(StatusCode::OK, filter.into_api()))
}
