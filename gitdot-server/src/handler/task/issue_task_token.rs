use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_api::{endpoint::task::issue_task_token as api, resource::task as resource};
use gitdot_core::dto::IssueTaskJwtRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn issue_task_token(
    _auth_user: Principal<User>,
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Result<AppResponse<api::IssueTaskTokenResponse>, AppError> {
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
