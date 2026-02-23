use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_api::endpoint::get_task as api;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn get_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<AppResponse<api::GetTaskResponse>, AppError> {
    state
        .task_service
        .get_task(id)
        .await
        .map_err(AppError::from)
        .map(|task| AppResponse::new(StatusCode::OK, task.into_api()))
}
