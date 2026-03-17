use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::repository::get_repository_blobs as api;
use gitdot_core::dto::{
    GetRepositoryBlobsRequest, RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{ClientCookies, Principal, User},
};

#[axum::debug_handler]
pub async fn get_repository_blobs(
    auth_user: Option<Principal<User>>,
    client_cookies: ClientCookies,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Json(params): Json<api::GetRepositoryBlobsRequest>,
) -> Result<AppResponse<api::GetRepositoryBlobsResponse>, AppError> {
    let request = RepositoryAuthorizationRequest::new(
        auth_user.map(|u| u.id),
        &owner,
        &repo,
        RepositoryPermission::Read,
    )?;
    state
        .authorization_service
        .verify_authorized_for_repository(request)
        .await?;

    if let Some(client_sha) = client_cookies.sha {
        let latest_sha = state
            .repo_service
            .resolve_ref_sha(&owner, &repo, &params.ref_name)
            .await
            .map_err(AppError::from)?;
        if latest_sha == client_sha {
            return Ok(AppResponse::NotModified);
        }
    }

    let request = GetRepositoryBlobsRequest::new(&repo, &owner, params.ref_name, params.paths)?;
    state
        .repo_service
        .get_repository_blobs(request)
        .await
        .map_err(AppError::from)
        .map(|blobs| AppResponse::new(StatusCode::OK, blobs.into_api()))
}
