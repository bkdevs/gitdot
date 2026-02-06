use axum::{
    body::Body,
    extract::{Path, State},
    http::HeaderMap,
};

use gitdot_core::dto::{
    GitHttpAuthorizationRequest, ProcessPushCommitsRequest, ReceivePackRequest, parse_ref_updates,
};

use crate::app::{AppError, AppState, AuthenticatedUser};
use crate::dto::GitHttpServerResponse;

#[axum::debug_handler]
pub async fn git_receive_pack(
    auth_user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    headers: HeaderMap,
    body: Body,
) -> Result<GitHttpServerResponse, AppError> {
    let user_id = auth_user.map(|u| u.id);
    let auth_request = GitHttpAuthorizationRequest::for_receive_pack(user_id, &owner, &repo)?;
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

    // Parse ref updates from the request body and spawn background task to process commits
    let ref_updates = parse_ref_updates(&body_bytes);
    if !ref_updates.is_empty() {
        let commit_service = state.commit_service.clone();
        let request = ProcessPushCommitsRequest {
            owner: owner.clone(),
            repo: repo.clone(),
            ref_updates,
        };
        tokio::spawn(async move {
            if let Err(e) = commit_service.process_push_commits(request).await {
                tracing::error!("Failed to process commits: {}", e);
            }
        });
    }

    Ok(response.into())
}
