use axum::{extract::State, http::StatusCode};

use gitdot_api::{endpoint::task::poll_task as api, resource::task as resource};
use gitdot_core::dto::IssueTaskJwtRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, RunnerToken},
};

#[axum::debug_handler]
pub async fn poll_task(
    State(state): State<AppState>,
    auth_runner: Principal<RunnerToken>,
) -> Result<AppResponse<api::PollTaskResponse>, AppError> {
    let Some(task) = state
        .task_service
        .poll_task(auth_runner.id)
        .await
        .map_err(AppError::from)?
    else {
        return Ok(AppResponse::new(StatusCode::OK, None));
    };

    let repository = state
        .repo_service
        .get_repository_by_id(task.repository_id)
        .await
        .map_err(AppError::from)?;

    let jwt = state
        .authentication_service
        .issue_task_token(IssueTaskJwtRequest {
            task_id: task.id,
            duration: std::time::Duration::from_secs(3600),
        })
        .await
        .map_err(AppError::from)?;

    Ok(AppResponse::new(
        StatusCode::OK,
        Some(resource::PollTaskResource {
            token: jwt.token,
            id: task.id,
            owner_name: repository.owner,
            repository_name: repository.name,
            s2_uri: task.s2_uri,
            name: task.name,
            command: task.command,
            status: task.status.into(),
        }),
    ))
}
