use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;

use gitdot_api::endpoint::repository::get_repository_resources as api;
use gitdot_core::dto::{
    GetRepositoryBlobsRequest, GetRepositoryPathsRequest, ListRepositoryCommitsRequest,
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
    let user_id = auth_user.map(|u| u.id);
    let auth_request =
        RepositoryAuthorizationRequest::new(user_id, &owner, &repo, RepositoryPermission::Read)?;
    state
        .authorization_service
        .verify_authorized_for_repository(auth_request)
        .await?;

    let head_sha = state
        .repo_service
        .resolve_ref_sha(&owner, &repo, "HEAD")
        .await
        .map_err(AppError::from)?;
    let now = Utc::now();

    let contents = async {
        if params.last_commit.as_deref() != Some(head_sha.as_str()) {
            return Ok::<_, AppError>(None);
        }
        let paths = state
            .repo_service
            .get_repository_paths(GetRepositoryPathsRequest::new(
                &repo,
                &owner,
                head_sha.clone(),
            )?)
            .await
            .map_err(AppError::from)?;
        let blob_paths: Vec<String> = paths.entries.iter().map(|e| e.path.clone()).collect();
        let blobs = state
            .repo_service
            .get_repository_blobs(GetRepositoryBlobsRequest::new(
                &repo,
                &owner,
                vec![head_sha.clone()],
                blob_paths,
            )?)
            .await
            .map_err(AppError::from)?;
        Ok(Some((paths, blobs)))
    };
    let commits = async {
        let fetch_all = params
            .last_updated
            .is_none_or(|lu| now - lu > chrono::Duration::hours(1));
        let (from, to) = if fetch_all {
            (None, None)
        } else {
            (params.last_updated, Some(now))
        };
        state
            .repo_service
            .list_repository_commits(ListRepositoryCommitsRequest::new(
                &owner,
                &repo,
                "HEAD".to_string(),
                from,
                to,
                None,
                None,
            )?)
            .await
            .map_err(AppError::from)
    };

    let (contents, commits) = tokio::try_join!(contents, commits)?;
    let (paths, blobs) = match contents {
        Some((paths, blobs)) => (Some(paths.into_api()), Some(blobs.into_api())),
        None => (None, None),
    };

    let resource = api::GetRepositoryResourcesResponse {
        last_commit: head_sha,
        last_updated: Some(now),
        paths,
        blobs,
        commits: Some(
            gitdot_api::resource::repository::RepositoryCommitsResource {
                commits: commits.into_api().data,
            },
        ),
    };
    Ok(AppResponse::new(StatusCode::OK, resource))
}
