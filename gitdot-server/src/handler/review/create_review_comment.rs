use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::create_review_comment as api;
use gitdot_core::dto::{CreateReviewCommentRequest, ReviewAuthorizationRequest};

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn create_review_comment(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number)): Path<(String, String, i32)>,
    Json(request): Json<api::CreateReviewCommentRequest>,
) -> Result<AppResponse<api::CreateReviewCommentResponse>, AppError> {
    let auth_request = ReviewAuthorizationRequest::new(auth_user.id, &owner, &repo, number)?;
    state
        .authorization_service
        .verify_authorized_for_review_comment(auth_request)
        .await?;

    let request = CreateReviewCommentRequest::new(
        &owner,
        &repo,
        number,
        auth_user.id,
        &request.body,
        request.diff_id,
        request.revision_id,
        request.parent_id,
        request.file_path.as_deref(),
        request.line_number,
        request.side.as_deref(),
    )?;
    state
        .review_service
        .create_comment(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::CREATED, response.into_api()))
}
