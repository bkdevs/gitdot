use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use chrono::Utc;

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
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<api::ListBuildsRequest>,
) -> Result<AppResponse<api::ListBuildsResponse>, AppError> {
    let now = Utc::now();
    let from = params
        .from
        .unwrap_or_else(|| now - chrono::Duration::weeks(2));
    let to = params.to.unwrap_or(now);
    let request = ListBuildsRequest::new(&owner, &repo, from, to)?;
    state
        .build_service
        .list_builds(request)
        .await
        .map_err(AppError::from)
        .map(|builds| AppResponse::new(StatusCode::OK, builds.builds.into_api()))
}
