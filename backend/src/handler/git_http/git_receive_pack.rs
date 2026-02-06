use axum::{
    body::Body,
    extract::{Path, State},
    http::HeaderMap,
};
use futures::TryStreamExt;
use tokio_util::io::StreamReader;

use gitdot_core::dto::{GitHttpAuthorizationRequest, ReceivePackRequest};

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

    let body_reader = StreamReader::new(
        body.into_data_stream()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
    );
    let request = ReceivePackRequest::new(&owner, &repo, content_type, Box::new(body_reader))?;
    let response = state.git_http_service.receive_pack(request).await?;
    Ok(response.into())
}
