use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::runner::create_runner_token as api;
use gitdot_core::dto::CreateRunnerTokenRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn refresh_runner_token(
    State(state): State<AppState>,
    _auth_user: Principal<User>,
    Path((owner, name)): Path<(String, String)>,
) -> Result<AppResponse<api::CreateRunnerTokenResponse>, AppError> {
    let request = CreateRunnerTokenRequest::new(&owner, &name)?;

    state
        .runner_service
        .refresh_runner_token(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::CREATED, r.into_api()))
}
