use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_api::endpoint::task::update_task as api;
use gitdot_core::dto::UpdateTaskRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<api::UpdateTaskRequest>,
) -> Result<AppResponse<api::UpdateTaskResponse>, AppError> {
    let request = UpdateTaskRequest::new(id, &request.status)?;
    state
        .task_service
        .update_task(request)
        .await
        .map_err(AppError::from)
        .map(|task| AppResponse::new(StatusCode::OK, task.into_api()))
}
