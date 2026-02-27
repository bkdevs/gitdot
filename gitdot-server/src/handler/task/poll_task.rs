use axum::{extract::State, http::StatusCode};

use gitdot_api::endpoint::task::poll_task as api;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::into_poll_api,
    extract::{Principal, Runner},
};

#[axum::debug_handler]
pub async fn poll_task(
    State(state): State<AppState>,
    auth_runner: Principal<Runner>,
) -> Result<AppResponse<api::PollTaskResponse>, AppError> {
    state
        .task_service
        .poll_task(auth_runner.id)
        .await
        .map_err(AppError::from)
        .map(|task| AppResponse::new(StatusCode::OK, task.map(into_poll_api)))
}
