use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::approve_review_diff as api;
use gitdot_core::dto::{ApproveReviewDiffRequest, ReviewingAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn approve_review_diff(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number, position)): Path<(String, String, i32, i32)>,
) -> Result<AppResponse<api::ApproveReviewDiffResponse>, AppError> {
    let auth_request = ReviewingAuthorizationRequest::new(auth_user.id, &owner, &repo, number)?;
    state
        .authorization_service
        .verify_authorized_for_reviewing(auth_request)
        .await?;

    let request = ApproveReviewDiffRequest::new(&owner, &repo, number, position, auth_user.id)?;
    let response = state.review_service.approve_review_diff(request).await?;

    Ok(AppResponse::new(StatusCode::OK, response.into_api()))
}
