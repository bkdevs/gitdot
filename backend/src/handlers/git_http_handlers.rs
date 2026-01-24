use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::HeaderMap,
};
use gitdot_core::dto::{InfoRefsRequest, ReceivePackRequest, UploadPackRequest};

use crate::app::{AppError, AppState};
use crate::dto::{GitHttpResponse, InfoRefsQuery};
use crate::utils::git::normalize_repo_name;

pub async fn git_info_refs(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<InfoRefsQuery>,
) -> Result<GitHttpResponse, AppError> {
    let request = InfoRefsRequest::new(&owner, &normalize_repo_name(&repo), &params.service)?;
    let response = state.git_http_service.info_refs(request).await?;

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
        .unwrap_or("")
        .to_string();

    let request = UploadPackRequest::new(
        &owner,
        &normalize_repo_name(&repo),
        content_type,
        body_bytes.to_vec(),
    )?;
    let response = state.git_http_service.upload_pack(request).await?;

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
        .unwrap_or("")
        .to_string();

    let request = ReceivePackRequest::new(
        &owner,
        &normalize_repo_name(&repo),
        content_type,
        body_bytes.to_vec(),
    )?;
    let response = state.git_http_service.receive_pack(request).await?;

    Ok(GitHttpResponse::from(response))
}
