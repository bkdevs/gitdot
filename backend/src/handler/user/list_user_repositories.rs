use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use api::repository::RepositoryEndpointResponse;
use gitdot_core::dto::ListUserRepositoriesRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::RepositoryResponseWrapper;

#[axum::debug_handler]
pub async fn list_user_repositories(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(user_name): Path<String>,
) -> Result<AppResponse<Vec<RepositoryEndpointResponse>>, AppError> {
    let viewer_id = auth_user.map(|u| u.id);
    let request = ListUserRepositoriesRequest::new(&user_name, viewer_id)?;
    state
        .user_service
        .list_repositories(request)
        .await
        .map_err(AppError::from)
        .map(|repos| {
            let response: RepositoryResponseWrapper = repos.into();
            AppResponse::new(StatusCode::OK, response.0)
        })
}
