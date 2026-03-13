use axum::{
    extract::{Json, Path},
    http::StatusCode,
};

use gitdot_core::dto::ProcessReviewRequest;

use crate::{
    app::{AppError, AppResponse},
    dto::{ProcessReviewServerRequest, ProcessReviewServerResponse, ReviewAction},
};

#[axum::debug_handler]
pub async fn process_review(
    Path((owner, repo)): Path<(String, String)>,
    Json(request): Json<ProcessReviewServerRequest>,
) -> Result<AppResponse<ProcessReviewServerResponse>, AppError> {
    let review_request = ProcessReviewRequest::new(
        &owner,
        &repo,
        &request.ref_name,
        request.new_sha,
        request.pusher_id,
    )?;

    let action = if review_request.is_new() {
        ReviewAction::Created
    } else {
        ReviewAction::Updated
    };

    Ok(AppResponse::new(
        StatusCode::OK,
        ProcessReviewServerResponse {
            review_number: 1,
            action,
        },
    ))
}
