use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use gitdot_api::endpoint::create_task as api;
use gitdot_core::dto::CreateTaskRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn create_task(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Json(request): Json<api::CreateTaskRequest>,
) -> Result<AppResponse<api::CreateTaskResponse>, AppError> {
    let request = CreateTaskRequest::new(
        &request.repo_owner,
        &request.repo_name,
        request.script,
        auth_user.id,
    )?;
    state
        .task_service
        .create_task(request)
        .await
        .map_err(AppError::from)
        .map(|task| AppResponse::new(StatusCode::CREATED, task.into_api()))
}
