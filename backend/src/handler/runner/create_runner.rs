use crate::{
    app::{AppError, AppResponse, AppState, AuthenticatedUser, Jwt},
    dto::IntoApi,
};
use axum::{
    extract::{Json, State},
    http::StatusCode,
};

use api::endpoint::create_runner as api;
use gitdot_core::dto::CreateRunnerRequest;

#[axum::debug_handler]
pub async fn create_runner(
    State(state): State<AppState>,
    auth_user: AuthenticatedUser<Jwt>,
    Json(request): Json<api::CreateRunnerRequest>,
) -> Result<AppResponse<api::CreateRunnerResponse>, AppError> {
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
        .map(|q| AppResponse::new(StatusCode::CREATED, q.into_api()))
}
