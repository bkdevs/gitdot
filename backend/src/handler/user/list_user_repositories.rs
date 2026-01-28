use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::ListUserRepositoriesRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::RepositoryServerResponse;

#[axum::debug_handler]
pub async fn list_user_repositories(
    _auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(user_name): Path<String>,
) -> Result<AppResponse<Vec<RepositoryServerResponse>>, AppError> {
    let request = ListUserRepositoriesRequest::new(&user_name)?;
    state
        .user_service
        .list_repositories(request)
        .await
        .map_err(AppError::from)
        .map(|repos| {
            AppResponse::new(
                StatusCode::OK,
                repos.into_iter().map(|r| r.into()).collect(),
            )
        })
}
