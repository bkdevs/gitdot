use axum::{
    body::Body,
    extract::{Path, State},
};
use futures::TryStreamExt;
use tokio_util::io::StreamReader;

use gitdot_core::dto::{RepositoryAuthorizationRequest, RepositoryPermission, UploadPackRequest};

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
    body: Body,
) -> Result<GitHttpServerResponse, AppError> {
    let user_id = auth_user.map(|u| u.id);
    let auth_request =
        RepositoryAuthorizationRequest::new(user_id, &owner, &repo, RepositoryPermission::Read)?;
    state
        .auth_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let body_reader = StreamReader::new(
        body.into_data_stream()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
    );
    let request = UploadPackRequest::new(&owner, &repo, &content_type, Box::new(body_reader))?;
    let response = state.git_http_service.upload_pack(request).await?;
    Ok(response.into())
}
