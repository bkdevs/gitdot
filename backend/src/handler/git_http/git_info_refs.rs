use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use gitdot_core::dto::{GitHttpAuthorizationRequest, InfoRefsRequest};

use crate::app::{AppError, AppState};
use crate::dto::{GitHttpServerResponse, InfoRefsQuery};

#[axum::debug_handler]
pub async fn git_info_refs(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<InfoRefsQuery>,
    headers: HeaderMap,
) -> Result<GitHttpServerResponse, AppError> {
    let auth_header = headers.get("authorization").and_then(|v| v.to_str().ok());
    let auth_request =
        GitHttpAuthorizationRequest::for_info_refs(auth_header, &owner, &repo, &params.service)?;
    state
        .auth_service
        .verify_authorized_for_git_http(auth_request)
        .await?;

    let request = InfoRefsRequest::new(&owner, &repo, &params.service)?;
    let response = state.git_http_service.info_refs(request).await?;
    Ok(response.into())
}
