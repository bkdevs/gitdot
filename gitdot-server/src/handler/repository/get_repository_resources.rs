use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;

use gitdot_api::endpoint::repository::get_repository_resources as api;
use gitdot_core::dto::{
    GetCommitsRequest, GetRepositoryBlobsRequest, GetRepositoryPathsRequest,
    RepositoryAuthorizationRequest, RepositoryPermission,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, Service, User, Vercel},
};

#[axum::debug_handler]
pub async fn get_repository_resources(
    _service: Service<Vercel>,
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Json(params): Json<api::GetRepositoryResourcesRequest>,
) -> Result<AppResponse<api::GetRepositoryResourcesResponse>, AppError> {
    let auth_request = RepositoryAuthorizationRequest::new(
        auth_user.map(|u| u.id),
        &owner,
        &repo,
        RepositoryPermission::Read,
    )?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let head_sha = state
        .repo_service
        .resolve_ref_sha(&owner, &repo, "HEAD")
        .await
        .map_err(AppError::from)?;

    if params.last_commit.as_deref() == Some(head_sha.as_str()) {
        return Ok(AppResponse::new(
            StatusCode::OK,
            api::GetRepositoryResourcesResponse {
                last_commit: head_sha,
                last_updated: params.last_updated,
                paths: None,
                commits: None,
                blobs: None,
            },
        ));
    }

    let paths_request = GetRepositoryPathsRequest::new(&repo, &owner, "HEAD".to_string())?;
    let paths = state
        .repo_service
        .get_repository_paths(paths_request)
        .await
        .map_err(AppError::from)?;

    let blob_paths: Vec<String> = paths.entries.iter().map(|e| e.path.clone()).collect();
    let blobs_request =
        GetRepositoryBlobsRequest::new(&repo, &owner, "HEAD".to_string(), blob_paths)?;

    let now = Utc::now();
    let commits_request = GetCommitsRequest::new(
        &owner,
        &repo,
        "HEAD".to_string(),
        params
            .last_updated
            .unwrap_or_else(|| now - chrono::Duration::days(365)),
        now,
    )?;

    let (blobs, commits) = tokio::try_join!(
        async {
            state
                .repo_service
                .get_repository_blobs(blobs_request)
                .await
                .map_err(AppError::from)
        },
        async {
            state
                .commit_service
                .get_commits(commits_request)
                .await
                .map_err(AppError::from)
        },
    )?;

    let resource = api::GetRepositoryResourcesResponse {
        last_commit: head_sha,
        last_updated: Some(now),
        paths: Some(paths.into_api()),
        commits: Some(commits.into_api()),
        blobs: Some(blobs.into_api()),
    };
    Ok(AppResponse::new(StatusCode::OK, resource))
}
