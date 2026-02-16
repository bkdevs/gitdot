use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_api::endpoint::get_repository_tree as api;
use gitdot_core::dto::{GetRepositoryTreeRequest, RepositoryAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::AuthenticatedUser,
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn get_repository_tree(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<api::GetRepositoryTreeRequest>,
) -> Result<AppResponse<api::GetRepositoryTreeResponse>, AppError> {
    let request = RepositoryAuthorizationRequest::new(auth_user.map(|u| u.id), &owner, &repo)?;
    state
        .auth_service
        .verify_authorized_for_repository(request)
        .await?;

    let request = GetRepositoryTreeRequest::new(&repo, &owner, params.ref_name)?;
    state
        .repo_service
        .get_repository_tree(request)
        .await
        .map_err(AppError::from)
        .map(|tree| AppResponse::new(StatusCode::OK, tree.into_api()))
}
