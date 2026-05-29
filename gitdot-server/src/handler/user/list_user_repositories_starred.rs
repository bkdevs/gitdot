use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::list_user_repositories_starred as api;
use gitdot_core::dto::ListUserStarredRepositoriesRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_user_starred_repositories(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path(user_name): Path<String>,
    Query(query): Query<api::ListUserStarredRepositoriesRequest>,
) -> Result<AppResponse<api::ListUserStarredRepositoriesResponse>, AppError> {
    let viewer_id = auth_user.map(|u| u.id);
    let request = ListUserStarredRepositoriesRequest::new(
        &user_name,
        viewer_id,
        query.cursor.as_deref(),
        query.limit,
    )?;
    state
        .user_service
        .list_starred_repositories(request)
        .await
        .map_err(AppError::from)
        .map(|page| AppResponse::new(StatusCode::OK, page.into_api()))
}
