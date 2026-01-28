use axum::extract::{Path, State};
use axum::http::StatusCode;

use gitdot_core::dto::GetUserRepositoriesRequest;

use crate::app::{AppError, AppResponse, AppState};
use crate::dto::RepositoryServerResponse;

#[axum::debug_handler]
pub async fn list_user_repositories(
    State(state): State<AppState>,
    Path(user_name): Path<String>,
) -> Result<AppResponse<Vec<RepositoryServerResponse>>, AppError> {
    let request = GetUserRepositoriesRequest::new(&user_name)?;
    state
        .user_service
        .get_repositories(request)
        .await
        .map_err(AppError::from)
        .map(|repos| {
            AppResponse::new(
                StatusCode::OK,
                repos.into_iter().map(|r| r.into()).collect(),
            )
        })
}
