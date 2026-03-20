use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::repository::get_repository_settings as api;
use gitdot_core::dto::{
    GetRepositorySettingsRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn get_repository_settings(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> Result<AppResponse<api::GetRepositorySettingsResponse>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(
        auth_user.map(|u| u.id),
        &owner,
        &repo,
        RepositoryPermission::Read,
    )?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = GetRepositorySettingsRequest::new(&owner, &repo)?;
    state
        .repo_service
        .get_repository_settings(request)
        .await
        .map_err(AppError::from)
        .map(|resp| AppResponse::new(StatusCode::OK, resp.into_api()))
}
