use axum::{
    extract::{Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::build::list_builds as api;
use gitdot_core::dto::ListBuildsRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_builds(
    _auth_user: Principal<User>,
    State(state): State<AppState>,
    Query(query): Query<api::ListBuildsRequest>,
) -> Result<AppResponse<api::ListBuildsResponse>, AppError> {
    let request = ListBuildsRequest::new(&query.owner, &query.repo)?;
    state
        .build_service
        .list_builds(request)
        .await
        .map_err(AppError::from)
        .map(|builds| AppResponse::new(StatusCode::OK, builds.into_api()))
}
