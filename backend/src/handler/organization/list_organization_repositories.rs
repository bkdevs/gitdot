use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::list_organization_repositories as api;
use gitdot_core::dto::ListOrganizationRepositoriesRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn list_organization_repositories(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path(org_name): Path<String>,
) -> Result<AppResponse<api::ListOrganizationRepositoriesResponse>, AppError> {
    let viewer_id = auth_user.map(|u| u.id);
    let request = ListOrganizationRepositoriesRequest::new(&org_name, viewer_id)?;
    state
        .org_service
        .list_repositories(request)
        .await
        .map_err(AppError::from)
        .map(|repos| AppResponse::new(StatusCode::OK, repos.into_api()))
}
