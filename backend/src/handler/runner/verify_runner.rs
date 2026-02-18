use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_api::endpoint::runner::verify_runner as api;
use gitdot_core::{dto::VerifyRunnerRequest, error::AuthorizationError};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, Runner},
};

#[axum::debug_handler]
pub async fn verify_runner(
    State(state): State<AppState>,
    auth_runner: Principal<Runner>,
    Path(id): Path<Uuid>,
) -> Result<AppResponse<api::VerifyRunnerResponse>, AppError> {
    if auth_runner.id != id {
        return Err(AppError::Authorization(AuthorizationError::Unauthorized));
    }

    let request = VerifyRunnerRequest { runner_id: id };

    state
        .runner_service
        .verify_runner(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::NO_CONTENT, ()))
}
