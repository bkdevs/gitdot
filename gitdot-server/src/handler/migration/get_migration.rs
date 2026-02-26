use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::migration::get_migration as api;
use gitdot_core::dto::GetMigrationRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn get_migration(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path(number): Path<i32>,
) -> Result<AppResponse<api::GetMigrationResponse>, AppError> {
    let request = GetMigrationRequest::new(auth_user.id, number);
    state
        .migration_service
        .get_migration(request)
        .await
        .map_err(AppError::from)
        .map(|m| AppResponse::new(StatusCode::OK, m.into_api()))
}
