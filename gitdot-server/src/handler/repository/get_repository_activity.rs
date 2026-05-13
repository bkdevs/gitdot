use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::get_repository_activity as api;
use gitdot_core::dto::{
    GetRepositoryActivityRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn get_repository_activity(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> Result<AppResponse<api::GetRepositoryActivityResponse>, AppError> {
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

    let request = GetRepositoryActivityRequest::new(&owner, &repo)?;
    state
        .repo_service
        .get_repository_activity(request)
        .await
        .map_err(AppError::from)
        .map(|events| AppResponse::new(StatusCode::OK, events.into_api()))
}
