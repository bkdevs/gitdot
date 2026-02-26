use axum::{extract::State, http::StatusCode};

use gitdot_api::endpoint::runner::verify_runner as api;
use gitdot_core::dto::VerifyRunnerRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, Runner},
};

#[axum::debug_handler]
pub async fn verify_runner(
    State(state): State<AppState>,
    auth_runner: Principal<Runner>,
) -> Result<AppResponse<api::VerifyRunnerResponse>, AppError> {
    let request = VerifyRunnerRequest {
        runner_id: auth_runner.id,
    };

    state
        .runner_service
        .verify_runner(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::OK, ()))
}
