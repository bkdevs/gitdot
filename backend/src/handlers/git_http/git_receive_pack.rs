use axum::{
    body::Body,
    extract::{Path, State},
    http::HeaderMap,
};
use gitdot_core::dto::ReceivePackRequest;

use crate::app::{AppError, AppState};
use crate::dto::GitHttpServerResponse;
use crate::utils::git::normalize_repo_name;

pub async fn git_receive_pack(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    headers: HeaderMap,
    body: Body,
) -> Result<GitHttpServerResponse, AppError> {
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
    Ok(response.into())
}
