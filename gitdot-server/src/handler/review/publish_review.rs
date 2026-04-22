use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::resource::review::ReviewResource;
use gitdot_core::dto::{PublishReviewRequest, ReviewAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn publish_review(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number)): Path<(String, String, i32)>,
) -> Result<AppResponse<ReviewResource>, AppError> {
    let auth_request = ReviewAuthorizationRequest::new(auth_user.id, &owner, &repo, number)?;
    state
        .authorization_service
        .verify_authorized_for_review(auth_request)
        .await?;

    let request = PublishReviewRequest::new(&owner, &repo, number)?;
    let review = state
        .review_service
        .publish_review(request)
        .await
        .map_err(AppError::from)?;

    Ok(AppResponse::new(StatusCode::OK, review.into_api()))
}
