use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::ListOrganizationRepositoriesRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::RepositoryServerResponse;

#[axum::debug_handler]
pub async fn list_organization_repositories(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path(org_name): Path<String>,
) -> Result<AppResponse<Vec<RepositoryServerResponse>>, AppError> {
    let viewer_id = auth_user.map(|u| u.id);
    let request = ListOrganizationRepositoriesRequest::new(&org_name, viewer_id)?;
    state
        .org_service
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
