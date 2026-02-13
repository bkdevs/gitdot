use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    app::{AppError, AppResponse, AppState, AuthenticatedUser},
    dto::IntoApi,
};
use gitdot_core::dto::ListUserRepositoriesRequest;

#[axum::debug_handler]
pub async fn list_user_repositories(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(user_name): Path<String>,
) -> Result<AppResponse<api::ListUserRepositoriesResponse>, AppError> {
    let viewer_id = auth_user.map(|u| u.id);
    let request = ListUserRepositoriesRequest::new(&user_name, viewer_id)?;
    state
        .user_service
        .list_repositories(request)
        .await
        .map_err(AppError::from)
        .map(|repos| AppResponse::new(StatusCode::OK, repos.into_api()))
}
