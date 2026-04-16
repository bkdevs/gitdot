use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::merge_review_diff as api;
use gitdot_core::dto::{
    CreateCommitsRequest, MergeReviewDiffRequest, ReviewAuthorizationRequest, ReviewResponse,
};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

use super::ReviewIdParam;

#[axum::debug_handler]
pub async fn merge_review_diff(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, id, position)): Path<(String, String, ReviewIdParam, i32)>,
) -> Result<AppResponse<api::MergeReviewDiffResponse>, AppError> {
    let auth_request = ReviewAuthorizationRequest::new(auth_user.id, &owner, &repo, id.0.clone())?;
    state
        .authorization_service
        .verify_authorized_for_review(auth_request)
        .await?;

    let request = MergeReviewDiffRequest::new(&owner, &repo, id.0, position)?;
    let response = state.review_service.merge_review_diff(request).await?;

    // Create commits for all merged diffs, including previously merged ones.
    // ON CONFLICT DO NOTHING in create_bulk skips commits that already exist.
    if let Some(commit_request) = create_commits_request(&owner, &repo, &response)? {
        state.commit_service.create_commits(commit_request).await?;
    }

    Ok(AppResponse::new(StatusCode::OK, response.into_api()))
}

fn create_commits_request(
    owner: &str,
    repo: &str,
    response: &ReviewResponse,
) -> Result<Option<CreateCommitsRequest>, AppError> {
    let merged_diffs: Vec<_> = response
        .diffs
        .iter()
        .filter(|d| d.status == "merged")
        .collect();

    let first = match merged_diffs.first() {
        Some(d) => d,
        None => return Ok(None),
    };
    let last = merged_diffs.last().unwrap();

    let (first_rev, last_rev) = match (first.revisions.first(), last.revisions.first()) {
        (Some(f), Some(l)) => (f, l),
        _ => return Ok(None),
    };

    let old_sha = first_rev.parent_hash.clone();
    let new_sha = last_rev.commit_hash.clone();
    let ref_name = format!("refs/heads/{}", response.target_branch);

    let diff_positions: HashMap<String, i32> = merged_diffs
        .iter()
        .filter_map(|d| {
            d.revisions
                .first()
                .map(|r| (r.commit_hash.clone(), d.position))
        })
        .collect();

    let request = CreateCommitsRequest::new(
        owner,
        repo,
        old_sha,
        new_sha,
        ref_name,
        Some(response.number),
        diff_positions,
    )?;

    Ok(Some(request))
}
