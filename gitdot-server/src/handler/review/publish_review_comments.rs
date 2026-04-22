use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use gitdot_api::endpoint::publish_review_comments as api;
use gitdot_core::dto::PublishReviewCommentsRequest;

use crate::{
    app::{AppError, AppResponse, AppState},
    dto::IntoApi,
    extract::{Principal, User},
};

#[axum::debug_handler]
pub async fn publish_review_comments(
    auth_user: Principal<User>,
    State(state): State<AppState>,
    Path((owner, repo, number)): Path<(String, String, i32)>,
    Json(request): Json<api::PublishReviewCommentsRequest>,
) -> Result<AppResponse<api::PublishReviewCommentsResponse>, AppError> {
    // TODO: authorization check

    let comments = request
        .comments
        .into_iter()
        .map(|c| {
            (
                c.diff_id,
                c.revision_id,
                c.body,
                c.file_path,
                c.line_number_start,
                c.line_number_end,
                c.start_character,
                c.end_character,
                c.side,
            )
        })
        .collect();

    let request = PublishReviewCommentsRequest::new(&owner, &repo, number, auth_user.id, comments)?;

    state
        .review_service
        .publish_review_comments(request)
        .await
        .map_err(AppError::from)
        .map(|response| AppResponse::new(StatusCode::CREATED, response.into_api()))
}
