use axum::extract::{Json, State};
use http::StatusCode;

use gitdot_api::endpoint::create_build as api;
use gitdot_core::dto::CreateBuildRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn create_build(
    State(state): State<AppState>,
    Json(request): Json<api::CreateBuildRequest>,
) -> Result<AppResponse<api::CreateBuildResponse>, AppError> {
    let request = CreateBuildRequest::new(
        &request.repo_owner,
        &request.repo_name,
        request.task_dependencies,
    )?;

    state
        .build_service
        .create_build(request)
        .await
        .map_err(AppError::from)
        .map(|d| AppResponse::new(StatusCode::CREATED, d.into_api()))
}
