use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};

use gitdot_core::dto::CreateRepositoryRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{CreateRepositoryServerRequest, CreateRepositoryServerResponse};

#[axum::debug_handler]
pub async fn create_repository(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Json(request): Json<CreateRepositoryServerRequest>,
) -> Result<AppResponse<CreateRepositoryServerResponse>, AppError> {
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
        .map(|repo| AppResponse::new(StatusCode::CREATED, repo.into()))
}
