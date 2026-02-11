use axum::extract::{Json, State};
use gitdot_core::dto::CreateRunnerRequest;
use http::StatusCode;
use uuid::Uuid;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::{CreateRunnerServerRequest, RunnerServerResponse},
};

#[axum::debug_handler]
pub async fn create_runner(
    State(state): State<AppState>,
    Json(request): Json<CreateRunnerServerRequest>,
) -> Result<AppResponse<RunnerServerResponse>, AppError> {
    let request = CreateRunnerRequest::new(
        &request.name,
        Uuid::max(), // TODO: auth
        &request.owner_name,
        &request.owner_type,
    )?;

    state
        .runner_service
        .create_runner(request)
        .await
        .map_err(AppError::from)
        .map(|q| AppResponse::new(StatusCode::CREATED, q.into()))
}
