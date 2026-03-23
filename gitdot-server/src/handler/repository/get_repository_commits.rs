use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use chrono::Utc;

use gitdot_api::endpoint::get_repository_commits as api;
use gitdot_core::{
    dto::{GetCommitsRequest, RepositoryAuthorizationRequest, RepositoryPermission},
    error::CommitError,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

// TODO: this does not support ref in request as of now, service ignores it.
#[axum::debug_handler]
pub async fn get_repository_commits(
    auth_user: Option<Principal<User>>,
    State(state): State<AppState>,
    Path((owner, repo)): Path<(String, String)>,
    Query(params): Query<api::GetRepositoryCommitsRequest>,
) -> Result<AppResponse<api::GetRepositoryCommitsResponse>, AppError> {
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

    if params.to.is_some() && params.from.is_none() {
        return Err(AppError::Commit(CommitError::InvalidDateRange(
            "`to` requires `from` to be set".into(),
        )));
    }

    let now = Utc::now();
    let from = params
        .from
        .unwrap_or_else(|| now - chrono::Duration::days(30));
    let to = params.to.unwrap_or(now);

    let request = GetCommitsRequest::new(&owner, &repo, params.ref_name, from, to)?;
    state
        .commit_service
        .get_commits(request)
        .await
        .map_err(AppError::from)
        .map(|commits| AppResponse::new(StatusCode::OK, commits.into_api()))
}
