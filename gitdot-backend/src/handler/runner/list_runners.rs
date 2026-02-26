use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::runner::list_runners as api;
use gitdot_core::dto::ListRunnersRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, UserJwt},
};

#[axum::debug_handler]
pub async fn list_runners(
    State(state): State<AppState>,
    _auth_user: Principal<UserJwt>,
    Path(owner): Path<String>,
) -> Result<AppResponse<api::ListRunnersResponse>, AppError> {
    let request = ListRunnersRequest::new(&owner)?;

    state
        .runner_service
        .list_runners(request)
        .await
        .map_err(AppError::from)
        .map(|runners| AppResponse::new(StatusCode::OK, runners.into_api()))
}
