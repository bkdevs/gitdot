use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::runner::delete_runner as api;
use gitdot_core::dto::DeleteRunnerRequest;

use crate::app::{AppError, AppResponse, AppState};

#[axum::debug_handler]
pub async fn delete_runner(
    State(state): State<AppState>,
    Path((owner, name)): Path<(String, String)>,
) -> Result<AppResponse<api::DeleteRunnerResponse>, AppError> {
    let request = DeleteRunnerRequest::new(&owner, &name)?;

    state
        .runner_service
        .delete_runner(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::NO_CONTENT, ()))
}
