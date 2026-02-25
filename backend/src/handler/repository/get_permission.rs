use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::resource::repository::RepositoryPermissionResource;
use gitdot_core::dto::GetRepositoryPermissionRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn get_permission(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> Result<AppResponse<RepositoryPermissionResource>, AppError> {
    let request = GetRepositoryPermissionRequest::new(auth_user.id, &owner, &repo)?;
    let response = state
        .auth_service
        .get_repository_permission(request)
        .await?;

    Ok(AppResponse::new(
        StatusCode::OK,
        RepositoryPermissionResource {
            permission: response.permission.into(),
        },
    ))
}
