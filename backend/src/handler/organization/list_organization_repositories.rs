use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::GetOrganizationRepositoriesRequest;

use crate::app::{AppError, AppResponse, AppState};
use crate::dto::RepositoryServerResponse;

#[axum::debug_handler]
pub async fn list_organization_repositories(
    State(state): State<AppState>,
    Path(org_name): Path<String>,
) -> Result<AppResponse<Vec<RepositoryServerResponse>>, AppError> {
    // TODO: implement auth

    let request = GetOrganizationRepositoriesRequest::new(&org_name)?;
    state
        .org_service
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
