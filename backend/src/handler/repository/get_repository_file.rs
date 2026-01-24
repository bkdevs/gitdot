use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};

use gitdot_core::dto::GetRepositoryFileRequest;

use crate::app::{AppError, AppResponse, AppState, AuthenticatedUser};
use crate::dto::{GetRepositoryFileQuery, GetRepositoryFileServerResponse};

#[axum::debug_handler]
pub async fn get_repository_file(
    _auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<GetRepositoryFileQuery>,
) -> Result<AppResponse<GetRepositoryFileServerResponse>, AppError> {
    let request = GetRepositoryFileRequest::new(&repo, &owner, params.ref_name, params.path)?;
    state
        .repo_service
        .get_repository_file(request)
        .await
        .map_err(AppError::from)
        .map(|file| AppResponse::new(StatusCode::OK, file.into()))
}
