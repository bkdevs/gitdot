use axum::extract::{Json, State};
use gitdot_core::dto::CreateRunnerRequest;
use http::StatusCode;

use crate::{
    app::{AppError, AppResponse, AppState, AuthenticatedUser},
    dto::{CreateRunnerServerRequest, CreateRunnerServerResponse},
};

#[axum::debug_handler]
pub async fn create_runner(
    State(state): State<AppState>,
    auth_user: AuthenticatedUser,
    Json(request): Json<CreateRunnerServerRequest>,
) -> Result<AppResponse<CreateRunnerServerResponse>, AppError> {
    let request = CreateRunnerRequest::new(
        &request.name,
        auth_user.id,
        &request.owner_name,
        &request.owner_type,
    )?;

    state
        .runner_service
        .create_runner(request)
        .await
        .map_err(AppError::from)
        .map(|q| AppResponse::new(StatusCode::CREATED, q.into()))
}
