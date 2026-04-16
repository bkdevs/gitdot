use axum::{
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_core::dto::{PublishReviewRequest, ReviewAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    extract::{Principal, User},
};

use super::ReviewIdParam;

#[axum::debug_handler]
pub async fn publish_review(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, id)): Path<(String, String, ReviewIdParam)>,
) -> Result<AppResponse<()>, AppError> {
    let auth_request = ReviewAuthorizationRequest::new(auth_user.id, &owner, &repo, id.0.clone())?;
    state
        .authorization_service
        .verify_authorized_for_review(auth_request)
        .await?;

    let request = PublishReviewRequest::new(&owner, &repo, id.0)?;
    state
        .review_service
        .publish_review(request)
        .await
        .map_err(AppError::from)
        .map(|_| AppResponse::new(StatusCode::NO_CONTENT, ()))
}
