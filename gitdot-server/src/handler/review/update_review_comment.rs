use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use uuid::Uuid;

use gitdot_api::endpoint::update_review_comment as api;
use gitdot_core::dto::{ReviewCommentAuthorizationRequest, UpdateReviewCommentRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn update_review_comment(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number, comment_id)): Path<(String, String, i32, Uuid)>,
    Json(request): Json<api::UpdateReviewCommentRequest>,
) -> Result<AppResponse<api::UpdateReviewCommentResponse>, AppError> {
    let auth_request = ReviewCommentAuthorizationRequest::new(auth_user.id, comment_id);
    state
        .authorization_service
        .verify_authorized_for_review_comment(auth_request)
        .await?;

    let request = UpdateReviewCommentRequest::new(
        &owner,
        &repo,
        number,
        comment_id,
        auth_user.id,
        request.body,
    )?;

    state
        .review_service
        .update_review_comment(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::OK, response.into_api()))
}
