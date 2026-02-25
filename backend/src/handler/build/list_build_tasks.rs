use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_api::endpoint::build::list_build_tasks as api;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_build_tasks(
    _auth_user: Principal<User>,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<AppResponse<api::ListBuildTasksResponse>, AppError> {
    state
        .build_service
        .list_build_tasks(id)
        .await
        .map_err(AppError::from)
        .map(|tasks| AppResponse::new(StatusCode::OK, tasks.into_api()))
}
