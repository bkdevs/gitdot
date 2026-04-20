use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::create_review_comment as api;
use gitdot_core::dto::CreateReviewCommentRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

use super::ReviewIdParam;

#[axum::debug_handler]
pub async fn create_review_comment(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, id)): Path<(String, String, ReviewIdParam)>,
    Json(request): Json<api::CreateReviewCommentRequest>,
) -> Result<AppResponse<api::CreateReviewCommentResponse>, AppError> {
    // TODO: authorization check

    let request = CreateReviewCommentRequest::new(
        &owner,
        &repo,
        id.0,
        auth_user.id,
        request.diff_id,
        request.revision_id,
        request.body,
        request.file_path,
        request.line_number_start,
        request.line_number_end,
        request.start_character,
        request.end_character,
        request.side.as_deref(),
    )?;

    state
        .review_service
        .create_review_comment(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::CREATED, response.into_api()))
}
