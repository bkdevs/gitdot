use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::repository::convert_readonly_repository as api;
use gitdot_core::dto::{
    ConvertReadonlyRepositoryRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn convert_readonly_repository(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> Result<AppResponse<api::ConvertReadonlyRepositoryResponse>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(
        Some(auth_user.id),
        &owner,
        &repo,
        RepositoryPermission::Admin,
    )?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = ConvertReadonlyRepositoryRequest::new(&owner, &repo)?;
    let repository = state
        .repo_service
        .convert_readonly_repository(request)
        .await?;

    Ok(AppResponse::new(StatusCode::OK, repository.into_api()))
}
