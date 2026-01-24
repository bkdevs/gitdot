use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_core::dto::GetRepositoryTreeRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{GetRepositoryTreeQuery, GetRepositoryTreeServerResponse};

#[axum::debug_handler]
pub async fn get_repository_tree(
    _auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<GetRepositoryTreeQuery>,
) -> Result<AppResponse<GetRepositoryTreeServerResponse>, AppError> {
    let request = GetRepositoryTreeRequest::new(&repo, &owner, params.ref_name)?;
    state
        .repo_service
        .get_repository_tree(request)
        .await
        .map_err(AppError::from)
        .map(|tree| AppResponse::new(StatusCode::OK, tree.into()))
}
