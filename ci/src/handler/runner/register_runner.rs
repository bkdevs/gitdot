use axum::extract::{Path, State};
use gitdot_core::dto::RegisterRunnerRequest;
use http::StatusCode;
use uuid::Uuid;

use crate::app::{AppError, AppResponse, AppState};

#[axum::debug_handler]
pub async fn register_runner(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<AppResponse<()>, AppError> {
    let request = RegisterRunnerRequest::new(id);

    state
        .runner_service
        .register_runner(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::NO_CONTENT, ()))
}
