use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::repository::get_repository as api;
use gitdot_core::dto::GetRepositoryRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn get_repository(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> Result<AppResponse<api::GetRepositoryResponse>, AppError> {
    let request = GetRepositoryRequest::new(&owner, &repo)?;
    state
        .repo_service
        .get_repository(request)
        .await
        .map_err(AppError::from)
        .map(|repo| AppResponse::new(StatusCode::OK, repo.into_api()))
}
