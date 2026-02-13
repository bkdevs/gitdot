use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use api::endpoint::update_task as api;
use gitdot_core::dto::UpdateTaskRequest;

#[axum::debug_handler]
pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<api::UpdateTaskRequest>,
) -> Result<AppResponse<api::UpdateTaskResponse>, AppError> {
    let request = UpdateTaskRequest::new(id, &body.status)?;
    state
        .task_service
        .update_task(request)
        .await
        .map_err(AppError::from)
        .map(|task| AppResponse::new(StatusCode::OK, task.into_api()))
}
