use axum::extract::{Json, Path, State};
use gitdot_core::dto::UpdateTaskRequest;
use http::StatusCode;
use uuid::Uuid;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::{TaskServerResponse, UpdateTaskServerRequest},
};

#[axum::debug_handler]
pub async fn update_task(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTaskServerRequest>,
) -> Result<AppResponse<TaskServerResponse>, AppError> {
    let request = UpdateTaskRequest::new(id, &body.status)?;

    let response = state.task_service.update_task(request).await?;

    Ok(AppResponse::new(StatusCode::OK, response.into()))
}
