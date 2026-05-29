use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::repository::update_repository as api;
use gitdot_core::dto::{
    RepositoryAuthorizationRequest, RepositoryPermission, UpdateRepositoryRequest,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn update_repository(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Json(request): Json<api::UpdateRepositoryRequest>,
) -> Result<AppResponse<api::UpdateRepositoryResponse>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(
        Some(auth_user.id),
        &owner,
        &repo,
        RepositoryPermission::Admin,
    )?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let core_request =
        UpdateRepositoryRequest::new(&owner, &repo, request.description, request.readonly)?;
    state
        .repo_service
        .update_repository(core_request)
        .await
        .map_err(AppError::from)
        .map(|repo| AppResponse::new(StatusCode::OK, repo.into_api()))
}
