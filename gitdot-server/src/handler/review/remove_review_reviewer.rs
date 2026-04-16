use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::{RemoveReviewReviewerRequest, ReviewAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn remove_review_reviewer(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number, reviewer_name)): Path<(String, String, i32, String)>,
) -> Result<AppResponse<()>, AppError> {
    let auth_request = ReviewAuthorizationRequest::new(auth_user.id, &owner, &repo, number)?;
    state
        .authorization_service
        .verify_authorized_for_review(auth_request)
        .await?;

    let request = RemoveReviewReviewerRequest::new(&owner, &repo, number, &reviewer_name)?;
    state
        .review_service
        .remove_review_reviewer(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::OK, ()))
}
