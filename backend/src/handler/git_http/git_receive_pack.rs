use axum::{
    body::Body,
    extract::{Path, State},
    http::HeaderMap,
};
use gitdot_core::dto::{GitHttpAuthorizationRequest, ReceivePackRequest};

use crate::app::{AppError, AppState};
use crate::dto::GitHttpServerResponse;

#[axum::debug_handler]
pub async fn git_receive_pack(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    headers: HeaderMap,
    body: Body,
) -> Result<GitHttpServerResponse, AppError> {
    let auth_header = headers.get("authorization").and_then(|v| v.to_str().ok());
    let auth_request = GitHttpAuthorizationRequest::for_receive_pack(auth_header, &owner, &repo)?;
    state
        .auth_service
        .verify_authorized_for_git_http(auth_request)
        .await?;

    let content_type = headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

    let request = ReceivePackRequest::new(&owner, &repo, content_type, body_bytes.to_vec())?;
    let response = state.git_http_service.receive_pack(request).await?;
    Ok(response.into())
}
