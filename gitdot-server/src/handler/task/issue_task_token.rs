use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_api::{endpoint::task::issue_task_token as api, resource::task as resource};
use gitdot_core::{
    dto::{IssueTaskJwtRequest, RepositoryAuthorizationRequest, RepositoryPermission},
    error::TaskError,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn issue_task_token(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Result<AppResponse<api::IssueTaskTokenResponse>, AppError> {
    let task = state
        .task_service
        .get_task(task_id)
        .await
        .map_err(AppError::from)?
        .ok_or_else(|| AppError::Task(TaskError::NotFound(task_id.to_string())))?;

    let repository = state
        .repo_service
        .get_repository_by_id(task.repository_id)
        .await
        .map_err(AppError::from)?;

    let auth_request = RepositoryAuthorizationRequest::new(
        Some(auth_user.id),
        &repository.owner,
        &repository.name,
        RepositoryPermission::Read,
    )?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let jwt = state
        .authentication_service
        .issue_task_token(IssueTaskJwtRequest {
            task_id,
            duration: std::time::Duration::from_secs(60),
        })
        .await
        .map_err(AppError::from)?;

    Ok(AppResponse::new(
        StatusCode::OK,
        resource::TaskTokenResource { token: jwt.token },
    ))
}
