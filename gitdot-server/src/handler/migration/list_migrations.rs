use axum::{extract::State, http::StatusCode};

use gitdot_api::endpoint::migration::list_migrations as api;
use gitdot_core::dto::ListMigrationsRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_migrations(
    auth_user: Principal<User>,
    State(state): State<AppState>,
) -> Result<AppResponse<api::ListMigrationsResponse>, AppError> {
    let request = ListMigrationsRequest::new(auth_user.id);
    state
        .migration_service
        .list_migrations(request)
        .await
        .map_err(AppError::from)
        .map(|migrations| AppResponse::new(StatusCode::OK, migrations.into_api()))
}
