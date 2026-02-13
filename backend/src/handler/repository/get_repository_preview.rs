use crate::{
    app::{AppError, AppResponse, AppState, AuthenticatedUser},
    dto::IntoApi,
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use api::endpoint::get_repository_preview as api;
use gitdot_core::dto::{GetRepositoryPreviewRequest, RepositoryAuthorizationRequest};

#[axum::debug_handler]
pub async fn get_repository_preview(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<api::GetRepositoryPreviewRequest>,
) -> Result<AppResponse<api::GetRepositoryPreviewResponse>, AppError> {
    let request = RepositoryAuthorizationRequest::new(auth_user.map(|u| u.id), &owner, &repo)?;
    state
        .auth_service
        .verify_authorized_for_repository(request)
        .await?;

    let request =
        GetRepositoryPreviewRequest::new(&repo, &owner, params.ref_name, params.preview_lines)?;
    state
        .repo_service
        .get_repository_preview(request)
        .await
        .map_err(AppError::from)
        .map(|tree| AppResponse::new(StatusCode::OK, tree.into_api()))
}
