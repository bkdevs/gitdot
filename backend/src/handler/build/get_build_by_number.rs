use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::build::get_build_by_number as api;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn get_build_by_number(
    _auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number)): Path<(String, String, i32)>,
) -> Result<AppResponse<api::GetBuildByNumberResponse>, AppError> {
    let (build, tasks) = state
        .build_service
        .get_build_with_tasks(&owner, &repo, number)
        .await
        .map_err(AppError::from)?;

    let response = api::GetBuildByNumberResponse {
        build: build.into_api(),
        tasks: tasks.into_api(),
    };

    Ok(AppResponse::new(StatusCode::OK, response))
}
