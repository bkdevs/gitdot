use axum::extract::{Path, Query, State};
use gitdot_core::dto::InfoRefsRequest;

use crate::app::{AppError, AppState};
use crate::dto::{GitHttpServerResponse, InfoRefsQuery};
use crate::utils::git::normalize_repo_name;

pub async fn git_info_refs(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<InfoRefsQuery>,
) -> Result<GitHttpServerResponse, AppError> {
    let request = InfoRefsRequest::new(&owner, &normalize_repo_name(&repo), &params.service)?;
    let response = state.git_http_service.info_refs(request).await?;
    Ok(response.into())
}
