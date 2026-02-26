use axum::{
    extract::{Path, State},
    http::StatusCode,
};

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
    Path((owner, repo, number)): Path<(String, String, i32)>,
) -> Result<AppResponse<api::ListBuildTasksResponse>, AppError> {
    let (_, tasks) = state
        .build_service
        .get_build_with_tasks(&owner, &repo, number)
        .await
        .map_err(AppError::from)?;

    Ok(AppResponse::new(StatusCode::OK, tasks.into_api()))
}
