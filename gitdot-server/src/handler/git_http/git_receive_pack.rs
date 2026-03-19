use axum::{
    body::Body,
    extract::{Path, State},
    http::HeaderMap,
};

use gitdot_core::dto::{ReceivePackRequest, RepositoryAuthorizationRequest, RepositoryPermission};

use super::create_body_reader;
use crate::{
    app::{AppError, AppState},
    dto::GitHttpServerResponse,
    extract::{ContentType, Principal, User},
};

#[axum::debug_handler]
pub async fn git_receive_pack(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    ContentType(content_type): ContentType,
    headers: HeaderMap,
    body: Body,
) -> Result<GitHttpServerResponse, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(
        Some(auth_user.id),
        &owner,
        &repo,
        RepositoryPermission::Write,
    )?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let body_reader = create_body_reader(&headers, body).await;
    let request = ReceivePackRequest::new(
        Some(auth_user.id),
        &owner,
        &repo,
        &content_type,
        body_reader,
    )?;
    let response = state.git_http_service.receive_pack(request).await?;
    Ok(response.into())
}
