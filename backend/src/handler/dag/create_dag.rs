use axum::extract::{Json, State};
use http::StatusCode;

use gitdot_api::endpoint::create_dag as api;
use gitdot_core::dto::CreateDagRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn create_dag(
    State(state): State<AppState>,
    Json(request): Json<api::CreateDagRequest>,
) -> Result<AppResponse<api::CreateDagResponse>, AppError> {
    let request =
        CreateDagRequest::new(&request.repo_owner, &request.repo_name, request.task_dependencies)?;

    state
        .dag_service
        .create_dag(request)
        .await
        .map_err(AppError::from)
        .map(|d| AppResponse::new(StatusCode::CREATED, d.into_api()))
}
