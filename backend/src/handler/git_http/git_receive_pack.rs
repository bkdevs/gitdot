use axum::{
    body::Body,
    extract::{Path, State},
};
use futures::TryStreamExt;
use tokio_util::io::StreamReader;

use gitdot_core::dto::{ReceivePackRequest, RepositoryAuthorizationRequest, RepositoryPermission};

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
    body: Body,
) -> Result<GitHttpServerResponse, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(
        Some(auth_user.id),
        &owner,
        &repo,
        RepositoryPermission::Write,
    )?;
    state
        .auth_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let body_reader = StreamReader::new(
        body.into_data_stream()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
    );
    let request = ReceivePackRequest::new(&owner, &repo, &content_type, Box::new(body_reader))?;
    let response = state.git_http_service.receive_pack(request).await?;
    Ok(response.into())
}
