use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::runner::get_runner as api;
use gitdot_core::dto::GetRunnerRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, UserJwt},
};

#[axum::debug_handler]
pub async fn get_runner(
    State(state): State<AppState>,
    _auth_user: Principal<UserJwt>,
    Path((owner, name)): Path<(String, String)>,
) -> Result<AppResponse<api::GetRunnerResponse>, AppError> {
    let request = GetRunnerRequest::new(&owner, &name)?;

    state
        .runner_service
        .get_runner(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::OK, r.into_api()))
}
