use axum::{
    extract::{Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::task::list_tasks as api;
use gitdot_core::dto::ListTasksRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_tasks(
    _auth_user: Principal<User>,
    State(state): State<AppState>,
    Query(query): Query<api::ListTasksRequest>,
) -> Result<AppResponse<api::ListTasksResponse>, AppError> {
    let request = ListTasksRequest::new(&query.owner, &query.repo)?;
    state
        .task_service
        .list_tasks(request)
        .await
        .map_err(AppError::from)
        .map(|tasks| AppResponse::new(StatusCode::OK, tasks.into_api()))
}
