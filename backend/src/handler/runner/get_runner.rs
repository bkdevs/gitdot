use axum::{
    extract::{Path, Query, State},
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
    Path(name): Path<String>,
    Query(query): Query<api::GetRunnerRequest>,
) -> Result<AppResponse<api::GetRunnerResponse>, AppError> {
    let request = GetRunnerRequest::new(&query.owner_name, &query.owner_type, &name)?;

    state
        .runner_service
        .get_runner(request)
        .await
        .map_err(AppError::from)
        .map(|r| AppResponse::new(StatusCode::OK, r.into_api()))
}
