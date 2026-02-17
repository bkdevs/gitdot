use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::{
    DeleteRepositoryRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::AuthenticatedUser,
};

#[axum::debug_handler]
pub async fn delete_repository(
    auth_user: AuthenticatedUser,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
) -> Result<AppResponse<()>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(
        Some(auth_user.id),
        &owner,
        &repo,
        RepositoryPermission::Write,
    )?;
    state
        .auth_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let request = DeleteRepositoryRequest::new(&owner, &repo)?;
    state.repo_service.delete_repository(request).await?;

    Ok(AppResponse::new(StatusCode::NO_CONTENT, ()))
}
