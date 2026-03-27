use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;

use gitdot_api::endpoint::repository::get_repository_resources as api;
use gitdot_core::dto::{
    GetCommitsRequest, GetRepositoryBlobsRequest, GetRepositoryPathsRequest, ListBuildsRequest,
    ListQuestionsRequest, ListReviewsRequest, RepositoryAuthorizationRequest, RepositoryPermission,
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

    if params.last_commit.as_deref() == Some(head_sha.as_str()) {
        return Ok(AppResponse::new(
            StatusCode::OK,
            api::GetRepositoryResourcesResponse {
                last_commit: head_sha,
                last_updated: params.last_updated,
                paths: None,
                commits: None,
                blobs: None,
                questions: None,
                reviews: None,
                builds: None,
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
        GetRepositoryBlobsRequest::new(&repo, &owner, vec!["HEAD".to_string()], blob_paths)?;

    let now = Utc::now();
    let commits_from = params
        .last_updated
        .unwrap_or_else(|| now - chrono::Duration::days(365));
    let resources_from = params
        .last_updated
        .unwrap_or_else(|| now - chrono::Duration::weeks(2));

    let commits_request =
        GetCommitsRequest::new(&owner, &repo, "HEAD".to_string(), commits_from, now)?;

    let questions_request = ListQuestionsRequest::new(&owner, &repo, user_id, resources_from, now)?;

    let reviews_request = ListReviewsRequest::new(&owner, &repo, user_id, resources_from, now)?;

    let builds_request = ListBuildsRequest::new(&owner, &repo, resources_from, now)?;

    let (blobs, commits, questions, reviews, builds) = tokio::try_join!(
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
                .question_service
                .list_questions(questions_request)
                .await
                .map_err(AppError::from)
        },
        async {
            state
                .review_service
                .list_reviews(reviews_request)
                .await
                .map_err(AppError::from)
        },
        async {
            state
                .build_service
                .list_builds(builds_request)
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
        questions: Some(questions.into_api()),
        reviews: Some(reviews.into_api()),
        builds: Some(builds.into_api()),
    };
    Ok(AppResponse::new(StatusCode::OK, resource))
}
