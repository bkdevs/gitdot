use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;

use gitdot_api::endpoint::repository::get_repository_resources as api;
use gitdot_core::dto::{
    GetCommitsRequest, GetRepositoryBlobsRequest, GetRepositoryPathsRequest,
    GetRepositorySettingsRequest,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
};

#[axum::debug_handler]
pub async fn get_repository_resources(
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Json(_params): Json<api::GetRepositoryResourcesRequest>,
) -> Result<AppResponse<api::GetRepositoryResourcesResponse>, AppError> {
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
        now - chrono::Duration::days(365),
        now,
    )?;

    let settings_request = GetRepositorySettingsRequest::new(&owner, &repo)?;

    let (blobs, commits, settings) = tokio::try_join!(
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
        async {
            state
                .repo_service
                .get_repository_settings(settings_request)
                .await
                .map_err(AppError::from)
        },
    )?;

    let resource = api::GetRepositoryResourcesResponse {
        paths: paths.into_api(),
        commits: commits.into_api(),
        blobs: blobs.into_api(),
        settings: settings.into_api(),
    };
    Ok(AppResponse::new(StatusCode::OK, resource))
}
