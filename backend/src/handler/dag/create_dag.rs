use axum::extract::{Json, State};
use gitdot_core::dto::CreateDagRequest;
use http::StatusCode;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::{CreateDagServerRequest, DagServerResponse},
};

#[axum::debug_handler]
pub async fn create_dag(
    State(state): State<AppState>,
    Json(request): Json<CreateDagServerRequest>,
) -> Result<AppResponse<DagServerResponse>, AppError> {
    let request = CreateDagRequest::new(&request.repo_owner, &request.repo_name, Vec::new())?;

    state
        .dag_service
        .create_dag(request)
        .await
        .map_err(AppError::from)
        .map(|d| AppResponse::new(StatusCode::CREATED, d.into()))
}
