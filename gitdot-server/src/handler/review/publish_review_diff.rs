use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::publish_review_diff as api;
use gitdot_core::dto::{PublishReviewDiffRequest, ReviewAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn publish_review_diff(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number, position)): Path<(String, String, i32, i32)>,
) -> Result<AppResponse<api::PublishReviewDiffResponse>, AppError> {
    let auth_request = ReviewAuthorizationRequest::new(auth_user.id, &owner, &repo, number)?;
    state
        .authorization_service
        .verify_authorized_for_review(auth_request)
        .await?;

    let request = PublishReviewDiffRequest::new(&owner, &repo, number, position)?;
    let response = state.review_service.publish_review_diff(request).await?;

    Ok(AppResponse::new(StatusCode::OK, response.into_api()))
}
