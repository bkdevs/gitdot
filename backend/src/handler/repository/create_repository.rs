use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::create_repository as api;
use gitdot_core::dto::{CreateRepositoryRequest, RepositoryCreationAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::AuthenticatedUser,
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn create_repository(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Json(request): Json<api::CreateRepositoryRequest>,
) -> Result<AppResponse<api::CreateRepositoryResponse>, AppError> {
    let auth_request =
        RepositoryCreationAuthorizationRequest::new(auth_user.id, &owner, &request.owner_type)?;
    state
        .auth_service
        .verify_authorized_for_repository_creation(auth_request)
        .await?;

    let request = CreateRepositoryRequest::new(
        &repo,
        auth_user.id,
        &owner,
        &request.owner_type,
        &request.visibility,
    )?;
    state
        .repo_service
        .create_repository(request)
        .await
        .map_err(AppError::from)
        .map(|repo| AppResponse::new(StatusCode::CREATED, repo.into_api()))
}
