use axum::{
    body::Body,
    extract::{Path, State},
    http::HeaderMap,
};

use gitdot_core::dto::{RepositoryAuthorizationRequest, RepositoryPermission, UploadPackRequest};

use super::create_body_reader;
use crate::{
    app::{AppError, AppState},
    dto::GitHttpServerResponse,
    extract::{ContentType, Principal, User},
};

#[axum::debug_handler]
pub async fn git_upload_pack(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    ContentType(content_type): ContentType,
    headers: HeaderMap,
    body: Body,
) -> Result<GitHttpServerResponse, AppError> {
    let user_id = auth_user.map(|u| u.id);
    let auth_request =
        RepositoryAuthorizationRequest::new(user_id, &owner, &repo, RepositoryPermission::Read)?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let body_reader = create_body_reader(&headers, body).await;
    let request = UploadPackRequest::new(&owner, &repo, &content_type, body_reader)?;
    let response = state.git_http_service.upload_pack(request).await?;
    Ok(response.into())
}
