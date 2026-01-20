use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::HeaderMap,
};

use crate::app::{AppError, AppState};
use crate::dto::{GitHttpResponse, InfoRefsQuery};

pub async fn git_info_refs(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<InfoRefsQuery>,
) -> Result<GitHttpResponse, AppError> {
    let response = state
        .git_http_service
        .info_refs(&owner, &repo, &params.service)
        .await?;

    Ok(GitHttpResponse::from(response))
}

pub async fn git_upload_pack(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    headers: HeaderMap,
    body: Body,
) -> Result<GitHttpResponse, AppError> {
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    let content_type = headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let response = state
        .git_http_service
        .upload_pack(&owner, &repo, content_type, &body_bytes)
        .await?;

    Ok(GitHttpResponse::from(response))
}

pub async fn git_receive_pack(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    headers: HeaderMap,
    body: Body,
) -> Result<GitHttpResponse, AppError> {
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    let content_type = headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let response = state
        .git_http_service
        .receive_pack(&owner, &repo, content_type, &body_bytes)
        .await?;

    Ok(GitHttpResponse::from(response))
}
