use crate::app::{AppError, AppResponse, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use api::endpoint::register_runner as api;
use gitdot_core::dto::RegisterRunnerRequest;

#[axum::debug_handler]
pub async fn register_runner(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<AppResponse<api::RegisterRunnerResponse>, AppError> {
    let request = RegisterRunnerRequest::new(id);

    state
        .runner_service
        .register_runner(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::NO_CONTENT, ()))
}
